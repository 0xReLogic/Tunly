# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Which versions are eligible for receiving such patches depends on the CVSS v3.0 Rating:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: [maedeeaew@gmail.com]

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the following information (as much as you can provide) to help us better understand the nature and scope of the possible issue:

* Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
* Full paths of source file(s) related to the manifestation of the issue
* The location of the affected source code (tag/branch/commit or direct URL)
* Any special configuration required to reproduce the issue
* Step-by-step instructions to reproduce the issue
* Proof-of-concept or exploit code (if possible)
* Impact of the issue, including how an attacker might exploit the issue

This information will help us triage your report more quickly.

## Preferred Languages

We prefer all communications to be in English.

## Security Update Process

When we receive a security bug report, we will:

1. Confirm the problem and determine the affected versions
2. Audit code to find any potential similar problems
3. Prepare fixes for all supported releases
4. Release new security fix versions as soon as possible

## Security Scanning

This project uses:

- **CodeQL** for static code analysis
- **Dependabot** for dependency vulnerability scanning
- **cargo-audit** for Rust security advisories
- **cargo-deny** for license and security policy enforcement
- **npm audit** for frontend dependency scanning

## Security Best Practices

When contributing to this project:

1. Never commit secrets, API keys, or credentials
2. Use environment variables for sensitive configuration
3. Follow secure coding practices
4. Keep dependencies up to date
5. Run security scans locally before submitting PRs

## Disclosure Policy

When we learn of a security vulnerability, we will:

1. Patch the vulnerability in a private repository
2. Release a security advisory
3. Credit the reporter (unless they wish to remain anonymous)
4. Publish the fix in a new release

## Comments on this Policy

If you have suggestions on how this process could be improved, please submit a pull request.
