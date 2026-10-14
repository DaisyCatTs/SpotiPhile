"""Regression checks for explicit upstream decision classification."""
import importlib.util
import pathlib
import unittest

spec = importlib.util.spec_from_file_location(
    "upstream_report", pathlib.Path(__file__).with_name("upstream-report.py")
)
report = importlib.util.module_from_spec(spec)
spec.loader.exec_module(report)


class Decisions(unittest.TestCase):
    def test_mentions_do_not_reject_and_deferred_stays_distinct(self):
        sha = "a" * 40
        self.assertEqual(report.ledger_decisions(f"Reviewed `{sha}`; still pending."), {})
        self.assertEqual(report.ledger_decisions(f"- Deferred: `{sha}` visual review."), {sha: "Deferred"})
        self.assertEqual(report.ledger_decisions(f"- Rejected: `{sha}` outside scope."), {sha: "Rejected"})

    def test_case_and_conflicting_decisions(self):
        sha = "b" * 40
        self.assertEqual(report.ledger_decisions(f" * deferred: `{sha.upper()}`"), {sha: "Deferred"})
        with self.assertRaises(ValueError):
            report.ledger_decisions(f"Deferred: {sha}\nRejected: {sha}")


if __name__ == "__main__":
    unittest.main()
