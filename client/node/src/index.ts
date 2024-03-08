import axios, { AxiosResponse } from 'axios';

enum Endpoints {
  ROOT = "/",
  SIZE = "/size",
  FILE_ISSUES = "/file/-/issues",
  FILE_COMMITS = "/file/-/commits",
  ISSUE_FILES = "/issue/-/files",
  ISSUE_COMMITS = "/issue/-/commits",
  COMMIT_FILES = "/commit/-/files",
  COMMIT_ISSUES = "/commit/-/issues",
  ISSUE_LIST = "/issue/list",
  AUTHOR_COMMITS = "/author/-/commits",
  AUTHORS_LIST = "/author/list",
  COMMIT_AUTHORS = "/commit/-/authors",
}

export class CupidoClient {
  private apiBaseUrl: string;

  constructor(apiBaseUrl: string = 'http://127.0.0.1:9410') {
    this.apiBaseUrl = apiBaseUrl;
  }

  private async getData(endpoint: string): Promise<any> {
    try {
      const response: AxiosResponse<any> = await axios.get<any>(`${this.apiBaseUrl}${endpoint}`);
      return response.data;
    } catch (error: any) {
      console.error('Error:', error.message);
      throw error;
    }
  }

  async api_root(): Promise<any> {
    return this.getData(Endpoints.ROOT);
  }

  async api_size(): Promise<any> {
    return this.getData(Endpoints.SIZE);
  }

  async api_fileIssues(file: string): Promise<string[]> {
    const endpoint = Endpoints.FILE_ISSUES + `?file=${encodeURIComponent(file)}`;
    return this.getData(endpoint);
  }

  async api_fileCommits(file: string): Promise<string[]> {
    const endpoint = Endpoints.FILE_COMMITS + `?file=${encodeURIComponent(file)}`;
    return this.getData(endpoint);
  }

  async api_issueFiles(issue: string): Promise<string[]> {
    const endpoint = Endpoints.ISSUE_FILES + `?issue=${encodeURIComponent(issue)}`;
    return this.getData(endpoint);
  }

  async api_issueCommits(issue: string): Promise<string[]> {
    const endpoint = Endpoints.ISSUE_COMMITS + `?issue=${encodeURIComponent(issue)}`;
    return this.getData(endpoint);
  }

  async api_commitFiles(commit: string): Promise<string[]> {
    const endpoint = Endpoints.COMMIT_FILES + `?commit=${encodeURIComponent(commit)}`;
    return this.getData(endpoint);
  }

  async api_commitIssues(commit: string): Promise<string[]> {
    const endpoint = Endpoints.COMMIT_ISSUES + `?commit=${encodeURIComponent(commit)}`;
    return this.getData(endpoint);
  }

  async api_issuesList(): Promise<string[]> {
    const endpoint = Endpoints.ISSUE_LIST;
    return this.getData(endpoint);
  }

  async api_authorRelatedCommits(author: string): Promise<string[]> {
    const endpoint = Endpoints.AUTHOR_COMMITS + `?author=${encodeURIComponent(author)}`;;
    return this.getData(endpoint);
  }

  async api_authorsList(): Promise<string[]> {
    const endpoint = Endpoints.AUTHORS_LIST;
    return this.getData(endpoint);
  }

  async api_commitRelatedAuthors(commit: string): Promise<string[]> {
    const endpoint = Endpoints.COMMIT_AUTHORS + `?commit=${encodeURIComponent(commit)}`;
    return this.getData(endpoint);
  }
}
