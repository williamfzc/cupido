use crate::collector::config::{Collect, CommitResult, Config};
use crate::relation::graph::RelationGraph;
use git2::{Commit, DiffOptions, Repository};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use regex::Regex;
use std::fmt::Write;

pub struct NativeCollector {}

impl Collect for NativeCollector {
    fn walk(&self, conf: Config) -> RelationGraph {
        let repo_path = &conf.repo_path;

        let repo = Repository::open(repo_path).expect("Failed to open repository");
        return walk_dfs(conf, &repo);
    }
}

fn walk_dfs(conf: Config, repo: &Repository) -> RelationGraph {
    let head = repo
        .head()
        .expect("Failed to get HEAD ref")
        .peel_to_commit()
        .expect("Failed to peel HEAD to commit");

    let mut revwalk = repo.revwalk().expect("Failed to create revwalk");
    revwalk.push(head.id()).expect("Failed to push commit");

    // top to bottom
    revwalk
        .set_sorting(git2::Sort::TIME)
        .expect("failed to set sorting");

    // only the first parent, for performance
    // good solution for large repo
    if !conf.multi_parents {
        revwalk
            .simplify_first_parent()
            .expect("failed to set simplify_first_parent");
    }

    let mut counter = 0;
    let mut graph = RelationGraph::new();
    graph.conf = conf.clone();

    let issue_regex: Regex = Regex::new(&*conf.issue_regex).unwrap();
    let pb = create_progress(conf.depth as u64);

    for id in revwalk {
        let commit_id = match id {
            Ok(commit_id) => commit_id,
            Err(_) => {
                eprintln!("Failed to get commit id");
                continue;
            }
        };

        let commit = match repo.find_commit(commit_id) {
            Ok(commit) => commit,
            Err(_) => {
                eprintln!("Failed to find commit {}", commit_id);
                continue;
            }
        };

        let commit_result = process_commit(&repo, &commit, &issue_regex, &conf);

        if commit_result.files.is_empty() {
            continue;
        }

        // files
        for file in &commit_result.files {
            graph.add_file_node(file);
        }

        // commits
        let commit_id_str = &commit_id.to_string();
        graph.add_commit_node(commit_id_str);
        for file in &commit_result.files {
            graph.add_edge_file2commit(file, commit_id_str);
        }

        // issues
        for issue in &commit_result.issues {
            graph.add_issue_node(issue);

            for file in &commit_result.files {
                graph.add_edge_file2issue(file, issue);
            }
            graph.add_edge_commit2issue(commit_id_str, issue);
        }

        // author
        let author_str = &commit.author().to_string();
        graph.add_author_node(author_str);
        graph.add_edge_author2commit(author_str, commit_id_str);

        counter += 1;
        if conf.progress {
            pb.inc(1);
        }
        if counter > conf.depth {
            break;
        }
    }

    return graph;
}

fn create_progress(size: u64) -> ProgressBar {
    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {items}/{total_items} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
    return pb;
}

fn process_commit(repo: &Repository, commit: &Commit, re: &Regex, conf: &Config) -> CommitResult {
    if let Some(parent) = commit.parent(0).ok() {
        // TODO: seems that we should do a cache here
        // libgit2 also has a cache too:
        // https://github.com/libgit2/libgit2/blob/9b2577f8e0ea5e412040566176636b26843ce67d/src/libgit2/object.c#L189
        let parent_tree = parent.tree().expect("Failed to get parent tree");
        let current_tree = commit.tree().expect("Failed to get commit tree");

        // https://libgit2.org/libgit2/#HEAD/type/git_diff_options
        let mut opts = DiffOptions::default();
        for each in &conf.path_specs {
            opts.pathspec(each);
        }
        opts.minimal(true);
        opts.include_unmodified(false);
        opts.include_ignored(false);
        opts.ignore_filemode(true);
        opts.force_text(true);

        // fast but not very fast ...
        // when the trees are large
        let changes = repo
            .diff_tree_to_tree(
                Some(&parent_tree),
                Some(&current_tree),
                Option::from(&mut opts),
            )
            .expect("Failed to get diff");
        let changed_files: Vec<String> = changes
            .deltas()
            .filter_map(|delta| {
                delta
                    .new_file()
                    .path()
                    .map(|path| path.to_string_lossy().into_owned())
            })
            .collect();

        // Issue extract
        let issues = re
            .find_iter(commit.message().unwrap_or_default())
            .map(|mat| mat.as_str().to_string())
            .collect();

        return CommitResult {
            files: changed_files,
            issues,
        };
    }
    return CommitResult::default();
}
