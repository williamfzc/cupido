import { CupidoClient } from '../src';

describe('CupidClient', () => {
  it('should fetch root data successfully', async () => {
    const client = new CupidoClient();
    const rootData = await client.api_root();
    expect(rootData).toBeDefined();
    // Add more assertions based on the expected structure or values of rootData
  });

  it('should fetch size data successfully', async () => {
    const client = new CupidoClient();
    const sizeData = await client.api_size();
    expect(sizeData).toBeDefined();
    // Add more assertions based on the expected structure or values of sizeData
  });

  it('should fetch file-related issues successfully', async () => {
    const client = new CupidoClient();
    const fileIssuesData = await client.api_fileIssues('src/server/app.rs');
    expect(fileIssuesData).toBeDefined();
    // Add more assertions based on the expected structure or values of fileIssuesData
  });

  it('should fetch file-related commits successfully', async () => {
    const client = new CupidoClient();
    const fileCommitsData = await client.api_fileCommits('src/server/app.rs');
    expect(fileCommitsData).toBeDefined();
    // Add more assertions based on the expected structure or values of fileCommitsData
  });

  it('should fetch issue-related files successfully', async () => {
    const client = new CupidoClient();
    const issueFilesData = await client.api_issueFiles('ISSUE123');
    expect(issueFilesData).toBeDefined();
    // Add more assertions based on the expected structure or values of issueFilesData
  });

  it('should fetch issue-related commits successfully', async () => {
    const client = new CupidoClient();
    const issueCommitsData = await client.api_issueCommits('ISSUE123');
    expect(issueCommitsData).toBeDefined();
    // Add more assertions based on the expected structure or values of issueCommitsData
  });

  it('should fetch commit-related files successfully', async () => {
    const client = new CupidoClient();
    const commitFilesData = await client.api_commitFiles('COMMIT456');
    expect(commitFilesData).toBeDefined();
    // Add more assertions based on the expected structure or values of commitFilesData
  });

  it('should fetch commit-related issues successfully', async () => {
    const client = new CupidoClient();
    const commitIssuesData = await client.api_commitIssues('COMMIT456');
    expect(commitIssuesData).toBeDefined();
    // Add more assertions based on the expected structure or values of commitIssuesData
  });

  it('should authors', async () => {
    const client = new CupidoClient();
    const commitIssuesData = await client.api_authorsList();
    expect(commitIssuesData).toBeDefined();
  });
});
