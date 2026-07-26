import path from "node:path";

export function soryPathOverride() {
  return (
    process.env.sory_EXECUTABLE ??
    path.join(process.cwd(), "..", "..", "sory-rs", "target", "debug", "sory")
  );
}
