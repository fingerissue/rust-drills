// 웹페이지에서 problem.rs를 grass 브랜치에 커밋하고, Actions 채점 결과를 조회하는 로직.
// 토큰은 브라우저 localStorage에만 저장되고 api.github.com 외에는 어디로도 전송되지 않는다.
// 필요한 fine-grained 토큰 권한(이 레포 한정): Contents(Read and write), Actions(Read-only)
const GH = { owner: 'fingerissue', repo: 'rust-drills', branch: 'grass', api: 'https://api.github.com' };

function ghHeaders(token) {
  return {
    'Authorization': 'Bearer ' + token,
    'Accept': 'application/vnd.github+json',
    'X-GitHub-Api-Version': '2022-11-28',
  };
}

function encodePath(path) {
  return path.split('/').map(encodeURIComponent).join('/');
}

function toBase64Utf8(str) {
  const bytes = new TextEncoder().encode(str);
  let bin = '';
  bytes.forEach(b => { bin += String.fromCharCode(b); });
  return btoa(bin);
}

async function ghError(res, what) {
  let detail = '';
  try { detail = (await res.json()).message || ''; } catch (e) { /* ignore */ }
  return new Error(`${what} 실패 (HTTP ${res.status}) ${detail}`.trim());
}

// 파일의 현재 sha (업데이트에 필요). 파일이 없으면 null.
async function getFileSha(token, path) {
  const url = `${GH.api}/repos/${GH.owner}/${GH.repo}/contents/${encodePath(path)}?ref=${GH.branch}`;
  const res = await fetch(url, { headers: ghHeaders(token) });
  if (res.status === 404) return null;
  if (!res.ok) throw await ghError(res, '파일 정보 조회');
  return (await res.json()).sha;
}

// 파일 내용을 grass 브랜치에 커밋하고 커밋 sha를 반환
async function commitFile(token, path, content, message) {
  const sha = await getFileSha(token, path);
  const body = { message, content: toBase64Utf8(content), branch: GH.branch };
  if (sha) body.sha = sha;
  const url = `${GH.api}/repos/${GH.owner}/${GH.repo}/contents/${encodePath(path)}`;
  const res = await fetch(url, {
    method: 'PUT',
    headers: { ...ghHeaders(token), 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  if (!res.ok) throw await ghError(res, '커밋');
  return (await res.json()).commit.sha;
}

// 특정 커밋에서 돌아간 Actions 실행을 찾는다 (없으면 null)
async function findRun(token, commitSha) {
  const url = `${GH.api}/repos/${GH.owner}/${GH.repo}/actions/runs?head_sha=${commitSha}&per_page=5`;
  const res = await fetch(url, { headers: ghHeaders(token) });
  if (!res.ok) throw await ghError(res, 'Actions 실행 조회');
  const data = await res.json();
  return (data.workflow_runs && data.workflow_runs[0]) || null;
}

// 채점이 끝날 때까지 폴링. onUpdate(문자열)로 진행 상황을 알려주고, 최종 결과 객체를 반환.
// 반환: { state: 'success' | 'failure' | 'timeout', url }
async function waitForGrade(token, commitSha, onUpdate, opts = {}) {
  const intervalMs = opts.intervalMs ?? 4000;
  const timeoutMs = opts.timeoutMs ?? 240000;
  const sleep = opts.sleep ?? (ms => new Promise(r => setTimeout(r, ms)));
  let waited = 0;
  while (waited <= timeoutMs) {
    const run = await findRun(token, commitSha);
    if (!run) {
      onUpdate('채점 대기 중... (Actions 시작 기다리는 중)');
    } else if (run.status !== 'completed') {
      onUpdate('채점 중... (' + run.status + ')');
    } else {
      return { state: run.conclusion === 'success' ? 'success' : 'failure', url: run.html_url };
    }
    await sleep(intervalMs);
    waited += intervalMs;
  }
  return { state: 'timeout', url: `https://github.com/${GH.owner}/${GH.repo}/actions` };
}

// node 테스트용 export (브라우저에서는 무시됨)
if (typeof module !== 'undefined') {
  module.exports = { GH, encodePath, toBase64Utf8, getFileSha, commitFile, findRun, waitForGrade };
}
