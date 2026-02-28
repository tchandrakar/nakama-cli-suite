# Step 014: Ticket Management

## Objective

Implement the `itachi create` and `itachi link` commands for creating Jira issues from the CLI, linking issues to Confluence pages, transitioning issues through workflow states, and performing bulk operations on multiple issues.

## Tasks

- [ ] Implement `itachi create <type> <summary>`:
  - Accept issue type (bug, story, task, epic, subtask) and summary as arguments
  - Support additional fields via flags:
    - `--project=PAY` or use default project from config
    - `--description="detailed description"`
    - `--assignee=alice`
    - `--priority=high`
    - `--labels=backend,api`
    - `--sprint=current` or `--sprint="Sprint 42"`
    - `--epic=PROJ-100`
    - `--parent=PROJ-142` (for subtasks)
  - Support interactive mode: if required fields are missing, prompt for them
  - Support natural language creation: `itachi create "bug where login fails on mobile"` (LLM extracts type, summary, description)
  - Validate fields before creation (project exists, assignee is valid, etc.)
  - Display created issue key and URL after success
- [ ] Implement `itachi link <issue> --doc <page>`:
  - Create a Jira remote link from the issue to the Confluence page
  - Support linking by page title or page ID
  - Support link types: `--type=documents|implements|references|relates-to`
  - Verify both the issue and page exist before linking
  - Support reverse linking (Confluence page links back to Jira issue)
- [ ] Implement issue transition:
  - `itachi transition <issue> <status>` or `itachi move <issue> <status>`
  - Map natural language status names to transition IDs
  - Support common transitions: "start", "review", "done", "close", "reopen"
  - Validate the transition is available for the current issue state
  - Support adding a comment with the transition: `--comment="Completed implementation"`
- [ ] Implement bulk operations:
  - `itachi bulk transition --jql="..." --to=Done` -- transition multiple issues
  - `itachi bulk assign --jql="..." --to=alice` -- reassign multiple issues
  - `itachi bulk label --jql="..." --add=urgent` -- add labels to multiple issues
  - Confirmation prompt before executing bulk operations (showing count)
  - Progress bar during execution
  - Summary report after completion (succeeded, failed, skipped)
- [ ] Implement issue templates:
  - Support issue templates in `.itachi/templates/`:
    ```yaml
    name: bug-report
    type: Bug
    project: PAY
    labels: [bug]
    description_template: |
      ## Steps to Reproduce
      1. ...
      ## Expected Behavior
      ## Actual Behavior
    ```
  - `itachi create --template=bug-report "Login fails on mobile"`
  - List templates: `itachi templates list`
- [ ] Write unit tests for field validation and natural language creation
- [ ] Write integration tests with mocked Jira API

## Acceptance Criteria

- `itachi create` creates issues with all specified fields
- Natural language creation correctly extracts issue type and details
- Interactive mode prompts for missing required fields
- `itachi link` creates bidirectional links between Jira and Confluence
- Issue transitions work with natural language status names
- Bulk operations execute with proper confirmation and progress reporting
- Issue templates simplify repeated creation patterns
- All operations validate input before API calls

## Dependencies

- Step 003 (Jira Client) must be complete for issue CRUD
- Step 004 (Confluence Client) must be complete for page resolution
- Step 005 (NL Query Translation) for natural language issue creation
- `nakama-ai` for natural language extraction
