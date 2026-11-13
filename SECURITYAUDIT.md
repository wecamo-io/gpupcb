# Security Audit Summary

## Preamble
The security audit is one of the two deliverables mentioned in subtask **1c. Documentation and security audit of created parser**.
The audit was carried out after completion of the prveious subtasks:
- **1a. Building hyperlynx importer in Rust to save the data in Rust structs**
- **1b. Simple importer GUI initialization for displaying parsed data. Precursor to the real GUI built in SolidJS and TypeScript to be used in Tauri**

Its main goal is to find potential vulnerabilities, security weaknesses, and configuration issues within the project repository. This is achieved using repository-scanning tools, such as OpenSSF Scorecard, to match the commonly accepted security standards and recommendations.

## What It Covers
The audit focuses primarily on identifying areas that might require improvement and providing recommendations to strengthen the overall security and maintainability of the project.

The following areas are included in the scope of the audit:

- **Dependency security:** Identify known vulnerabilities in dependencies used by the hyperlynx-parser library crate and the Tauri application. This includes both Rust dependencies managed through Cargo and TypeScript dependencies managed through pnpm.
- **Repository security practices:** Evaluate the repository's security practices, identifying potential weaknesses in areas such as dependency management, code review, security policies, and CI/CD configuration.
- **File selection restrictions (Tauri application):** Verify that users can select only HyperLynx files through the GUI file selection dialogs. The application should restrict file selection to supported HyperLynx file types (.hyp, .HYP) rather than allowing arbitrary files, particularly sensitive system files, to be selected for parsing.

## OpenSSF Scorecard
### 1. Initial scan
#### RESULTS
Aggregate score: 2.8 / 10

Check scores:
|  SCORE  |          NAME          |                                      REASON                                       |                                              DOCUMENTATION / REMEDIATION                                              |
|---------|------------------------|-----------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| 9 / 10  | Binary-Artifacts       | binaries present in source code                                                   | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#binary-artifacts       |
| 0 / 10  | Branch-Protection      | branch protection not enabled on development/release branches                     | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#branch-protection      |
| 10 / 10 | CI-Tests               | 3 out of 3 merged PRs checked by a CI test -- score normalized to 10              | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#ci-tests               |
| 0 / 10  | CII-Best-Practices     | no effort to earn an OpenSSF best practices badge detected                        | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#cii-best-practices     |
| 3 / 10  | Code-Review            | Found 1/3 approved changesets -- score normalized to 3                            | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#code-review            |
| 0 / 10  | Contributors           | project has 0 contributing companies or organizations -- score normalized to 0    | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#contributors           |
| 10 / 10 | Dangerous-Workflow     | no dangerous workflow patterns detected                                           | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#dangerous-workflow     |
| 0 / 10  | Dependency-Update-Tool | no update tool detected                                                           | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#dependency-update-tool |
| 0 / 10  | Fuzzing                | project is not fuzzed                                                             | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#fuzzing                |
| 10 / 10 | License                | license file detected                                                             | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#license                |
| 0 / 10  | Maintained             | project was created within the last 90 days. Please review its contents carefully | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#maintained             |
| ?       | Packaging              | packaging workflow not detected                                                   | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#packaging              |
| 0 / 10  | Pinned-Dependencies    | dependency not pinned by hash detected -- score normalized to 0                   | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#pinned-dependencies    |
| 0 / 10  | SAST                   | SAST tool is not run on all commits -- score normalized to 0                      | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#sast                   |
| 0 / 10  | Security-Policy        | security policy file not detected                                                 | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#security-policy        |
| 0 / 10  | Signed-Releases        | Project has not signed or included provenance with any releases.                  | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#signed-releases        |
| 0 / 10  | Token-Permissions      | detected GitHub workflow tokens with excessive permissions                        | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#token-permissions      |
| 5 / 10  | Vulnerabilities        | 5 existing vulnerabilities detected                                               | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#vulnerabilities        |

### 2. Fixing *Dependency-Update-Tool* check
Created dependabot.yaml file in .github/ directory to enable automatic dependency updates. This fixed the Scorecard check:

#### RESULTS
Aggregate score: 10.0 / 10

Check scores:

|  SCORE  |          NAME          |        REASON        |                              DETAILS                              |                                              DOCUMENTATION / REMEDIATION                                              |
|---------|------------------------|----------------------|-------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| 10 / 10 | Dependency-Update-Tool | update tool detected | Info: detected update tool: Dependabot: .github/dependabot.yaml:1 | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#dependency-update-tool |

### 3. Fixing *Pinned-Dependencies* check
Pinned dependencies used in **HyperLynx Parser CI** and **publish** workflows to use commit SHAs pointing to certain version of actions. This fixed the Scorecard check:

#### RESULTS

Aggregate score: 10.0 / 10

Check scores:
|  SCORE  |        NAME         |           REASON            |                              DETAILS                               |                                            DOCUMENTATION / REMEDIATION                                             |
|---------|---------------------|-----------------------------|--------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------|
| 10 / 10 | Pinned-Dependencies | all dependencies are pinned | Info:   4 out of   4 GitHub-owned GitHubAction dependencies pinned | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#pinned-dependencies |
|         |                     |                             | Info:   4 out of   4 third-party GitHubAction dependencies pinned  |

### 4. Fixing *Security-Policy* check
Created **SECURITY.md** file for GitHub. This fixed the Scorecard check:

#### RESULTS
Aggregate score: 10.0 / 10

Check scores:
|  SCORE  |      NAME       |            REASON             |                                          DOCUMENTATION / REMEDIATION                                           |
|---------|-----------------|-------------------------------|----------------------------------------------------------------------------------------------------------------|
| 10 / 10 | Security-Policy | security policy file detected | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#security-policy |

### 5. Fixing *Security-Policy* check
Added top-level read permissions to *HyperLynx Parser CI* and *publish* GitHub workflows. This fixed the Scorecard check:

#### RESULTS

Aggregate score: 10.0 / 10

Check scores:
|  SCORE  |       NAME        |                           REASON                           |                                            DETAILS                                             |                                           DOCUMENTATION / REMEDIATION                                            |
|---------|-------------------|------------------------------------------------------------|------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------|
| 10 / 10 | Token-Permissions | GitHub workflow tokens follow principle of least privilege | Warn: jobLevel 'contents' permission set to 'write': .github/workflows/publish.yaml:16         | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#token-permissions |
|         |                   |                                                            | Info: topLevel 'contents' permission set to 'read': .github/workflows/hyperlynx_parser.yaml:11 |                                                                                                                  |
|         |                   |                                                            | Info: topLevel 'contents' permission set to 'read': .github/workflows/publish.yaml:11          |                                                                                                                  |

Note: the following ***Warn: jobLevel ‘contents’ permission set to ‘write’: .github/workflows/publish.yaml:16*** points at the necessary write permission of *publish* workflow release-creating job.

### 6. Fixing *SAST* check
Created CodeQL workflow that runs on pull request to main branch or when pushing changes on feat/* branches. This fixed the Scorecard check:

#### RESULTS
Aggregate score: 10.0 / 10

Check scores:
|  SCORE  | NAME |           REASON           |                    DETAILS                    |                                     DOCUMENTATION / REMEDIATION                                     |
|---------|------|----------------------------|-----------------------------------------------|-----------------------------------------------------------------------------------------------------|
| 10 / 10 | SAST | SAST tool detected: CodeQL | Info: SAST configuration detected: CodeQL     | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#sast |
|         |      |                            | Warn: no pull requests merged into dev branch |                                                                                                     |

### 7. Fixing *Vulnerabilities* check
Started from:

#### RESULTS
Aggregate score: 5.0 / 10

Check scores:
| SCORE  |      NAME       |               REASON                |                              DETAILS                              |                                          DOCUMENTATION / REMEDIATION                                           |
|--------|-----------------|-------------------------------------|-------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| 5 / 10 | Vulnerabilities | 5 existing vulnerabilities detected | Warn: Project is vulnerable to: https://osv.dev/RUSTSEC-2024-0429 | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#vulnerabilities |
|        |                 |                                     | Warn: Project is vulnerable to: https://osv.dev/RUSTSEC-2024-0370 |                                                                                                                |
|        |                 |                                     | Warn: Project is vulnerable to: https://osv.dev/RUSTSEC-2026-0371 |                                                                                                                |
|        |                 |                                     | Warn: Project is vulnerable to: https://osv.dev/RUSTSEC-2026-0370 |                                                                                                                |
|        |                 |                                     | Warn: Project is vulnerable to: https://osv.dev/RUSTSEC-2026-0372 |                                                                                                                |


Investigated and updated the project's Rust dependencies to eliminate reported security vulnerabilities.

It turned out that three of the vulnerabilities had been connected to presence of unused **tauri-plugin-opener=2** dependency in src-tauri/Cargo.toml (which itself used **zbus** crate that had not yet been updated to use version 6 of it where those vulnerabilities would have been eradicated).

Deleted the **tauri-plugin-opener=2** because it was not used anywhere anyway.
It reduced the count of existing vulnerabilities to 2.

As for those vulnerabilities that are still left:

#### RESULTS
Aggregate score: 8.0 / 10

Check scores:
| SCORE  |      NAME       |               REASON                |                              DETAILS                              |                                          DOCUMENTATION / REMEDIATION                                           |
|--------|-----------------|-------------------------------------|-------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| 8 / 10 | Vulnerabilities | 2 existing vulnerabilities detected | Warn: Project is vulnerable to: https://osv.dev/RUSTSEC-2024-0429 | https://github.com/ossf/scorecard/blob/61e84743356275f6ffca23b1b2f2cb945e19eb96/docs/checks.md#vulnerabilities |
|        |                 |                                     | Warn: Project is vulnerable to: https://osv.dev/RUSTSEC-2024-0370 |                                                                                                                |

**RUSTSEC-2024-0429** can be resolved by updating Tauri's glib dependency from version 0.18.5 to 0.20.0 or newer. Since this dependency is managed by Tauri, the issue requires an upstream Tauri update or patch that introduces a compatible version of glib.

**RUSTSEC-2024-0370** concerns proc-macro-error, an unmaintained crate that is also included in Tauri's dependencies. Since no fixed version is available, resolving this warning requires Tauri to replace it with a maintained alternative.

Additionally, cargo audit was executed in both /crates/hyperlynx-parser and /src-tauri to check for known vulnerabilities in their respective Rust dependencies.

The results showed that:

- HyperLynx parser (/crates/hyperlynx-parser): No known vulnerabilities were detected.
- Tauri (/src-tauri): The audit confirmed the same dependency warnings previously identified by OpenSSF Scorecard: 
  - RUSTSEC-2024-0429
  - RUSTSEC-2024-0370.

## HyperLynx parser crate dependency update
Updated winnow from v1.0.4 to v1.0.5 and test-case from 3.4.0 to 3.4.1.
