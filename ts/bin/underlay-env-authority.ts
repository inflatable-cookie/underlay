#!/usr/bin/env bun

/**
 * Published CLI entry for consumer env/secret-authority conformance.
 * Imports the public `@inflatable-cookie/underlay/tools/env-authority` surface.
 */

import { runEnvAuthorityCli } from '../src/tools/env-authority.js';

runEnvAuthorityCli();
