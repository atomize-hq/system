import importlib.util
import pathlib
import unittest


ROOT = pathlib.Path(__file__).resolve().parents[1]
HARNESS_PATH = ROOT / "tools" / "harness.py"


def load_harness_module():
    spec = importlib.util.spec_from_file_location("legacy_harness", HARNESS_PATH)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


class LegacyHarnessOverlayDiscoveryTests(unittest.TestCase):
    def test_list_overlays_excludes_examples_subtree(self):
        harness = load_harness_module()

        overlays = harness.list_overlays()

        self.assertIn("task/research_discovery", overlays)
        self.assertNotIn("examples/overlay.example", overlays)


if __name__ == "__main__":
    unittest.main()
