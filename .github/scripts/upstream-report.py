"""Read-only upstream inventory. Never executes or integrates upstream code."""

import pathlib
import re
import subprocess

BASELINE = "995c768dcba4da7f93302bf2107af6b7ec3df02b"


def git(*args):
    try:
        return subprocess.check_output(["git", *args], text=True, timeout=60).strip()
    except subprocess.TimeoutExpired as error:
        raise SystemExit("Upstream inventory Git command timed out after 60 seconds.") from error


def main():
    accepted = set(re.findall(r"\b[0-9a-f]{40}\b", pathlib.Path(
        "docs/upstream/ACCEPTED_UPSTREAM.md"
    ).read_text(encoding="utf-8")))
    rejected = set(re.findall(r"\b[0-9a-f]{40}\b", pathlib.Path(
        "docs/upstream/REJECTED_UPSTREAM.md"
    ).read_text(encoding="utf-8")))
    log = git("log", "--reverse", "--format=%H%x09%s", f"{BASELINE}..upstream/main")
    print("# Spotifast upstream review")
    print(f"\nImmutable baseline: `{BASELINE}`")
    print(f"\nFetched upstream: `{git('rev-parse', 'upstream/main')}`")
    print("\nThis report does not merge, port, publish or approve changes.")
    print("\n| Commit | Change | Ledger status |\n| --- | --- | --- |")
    for entry in log.splitlines():
        sha, title = entry.split("\t", 1)
        title = title.replace("|", "\\|").replace("<", "&lt;").replace(">", "&gt;")
        status = "Accepted" if sha in accepted else "Rejected" if sha in rejected else "Pending review"
        print(f"| [{sha[:7]}](https://github.com/crmne/spotifast/commit/{sha}) | {title} | {status} |")


if __name__ == "__main__":
    main()
