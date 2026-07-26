import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { afterEach, beforeEach } from "@jest/globals";

const originalsoryHome = process.env.sory_HOME;
let currentsoryHome: string | undefined;

beforeEach(async () => {
  currentsoryHome = await fs.mkdtemp(path.join(os.tmpdir(), "sory-sdk-test-"));
  process.env.sory_HOME = currentsoryHome;
});

afterEach(async () => {
  const soryHomeToDelete = currentsoryHome;
  currentsoryHome = undefined;

  if (originalsoryHome === undefined) {
    delete process.env.sory_HOME;
  } else {
    process.env.sory_HOME = originalsoryHome;
  }

  if (soryHomeToDelete) {
    await fs.rm(soryHomeToDelete, { recursive: true, force: true });
  }
});
