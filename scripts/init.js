const fs = require('fs');
const path = require('path');
const { exec } = require('child_process');
const { promisify } = require('util');

async function main() {
  const { stdout, stderr } = await promisify(exec)('git remote -v');
  if (stderr) {
    console.error(stderr);
    process.exit(1);
  }

  const remotes = stdout.split('\n').filter((line) => line.includes('origin'));
  const repo = remotes[0].match(/github\.com:(.*)\.git\s/)[1];
  if (!repo) {
    console.error(`You should have a git remote origin on github`);
    process.exit(1);
  }

  const userName =
    (await promisify(exec)('git config --get user.name')).stdout.trim() ||
    repo.split('/')[0];
  const repoName = repo.split('/')[1];
  updatePackageJson(repoName, userName, repo);
  updateReadme(repoName, repo);
}

function updateReadme(repoName, repo) {
  fs.writeFileSync(
    path.join(__dirname, '..', 'README.md'),
    `# ${repoName}

[![codecov](https://codecov.io/gh/${repo}/branch/master/graph/badge.svg)](https://codecov.io/gh/${repo})
`,
  );
}

function updatePackageJson(repoName, userName, repo) {
  const pkg = getPackageJson(repoName, userName, repo);
  fs.writeFileSync(
    path.join(__dirname, '..', 'package.json'),
    JSON.stringify(pkg, null, 2) + '\n',
  );
}

function getPackageJson(repoName, userName, repo) {
  const pkg = JSON.parse(
    fs.readFileSync(path.join(__dirname, '..', 'package.json')),
  );
  pkg.name = repoName;
  pkg.author = userName;
  pkg.repository.url = `git+https://github.com/${repo}.git`;
  pkg.bugs.url = `https://github.com/${repo}/issues`;
  pkg.homepage = `https://github.com/${repo}#readme`;
  return pkg;
}

main();
