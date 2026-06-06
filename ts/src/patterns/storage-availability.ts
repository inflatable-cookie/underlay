import { BROWSER } from "esm-env";

export function isStorageAvailable(
  type: "localStorage" | "sessionStorage",
): boolean {
  if (!BROWSER) return false;

  try {
    const storage = window[type];
    const testKey = "__underlay_storage_test__";
    storage.setItem(testKey, "test");
    storage.removeItem(testKey);
    return true;
  } catch {
    return false;
  }
}
