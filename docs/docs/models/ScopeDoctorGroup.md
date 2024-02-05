---
sidebar_position: 1
---

# ScopeDoctorGroup

A "group" is a set of operations that should be preformed as part of solving a problem.
For example, there may be a group to install and configure Node, or other language.

A Group is defined by one or more "actions".
Taking a look at an example

```yaml
apiVersion: scope.github.com/v1alpha
kind: ScopeDoctorGroup
metadata:
  name: node
spec:
  description: Check node is ready.
  needs:
    - python
    - brew
  actions:
    - description: Node Version
      check:
        paths:
          - '.npmrc'
        commands:
          - ./scripts/check-node-version.sh
      fix:
        commands:
          - ./scripts/fix-node-version.sh
        helpText: Running into errors reach out in #foo-help
        helpUrl: https://go.example.com/get-help
      required: false
    - description: Install packages
      check:
        paths:
          - 'package.json'
          - '**/package.json'
          - yarn.lock
          - '**/yarn.lock'
      fix:
        commands:
          - yarn install
```

In the example above, there are two actions, the first ensures that node is operational.

## Actions

An action is a discrete step, they run in order defined.
The action should be atomic, and target a single resolution.

Notice an action can provide both `paths` and `commands`, if either of them indicate that the fix should run, it will run.
In the event there are no defined check, the fix will _always_ run.

## Fix

When the checks determine that something isn't correct, a fix is the way to automate the resolution.
When provided, `scope` will run them in order.

## Commands

A command can either be relative, or use the PATH.
To target a script relative to the group it must start with `.`, and giving a relative path to the group file.

## Schema

- `.spec.action` a series of steps to check and fix for the group.
- `.spec.needs[]` an array of names required before this group can run.
- `.spec.action[].required` default true, when true action failing check & fix will stop all execution. 
- `.spec.action[].check.paths` A list of globs to check for a change.
- `.spec.action[].check.commands` A list of commands to execute, using the exit code to determine if changes are needed.
- `.spec.action[].fix.commands` A list of commands to execute, attempting to resolve issues detected by the action.
- `.spec.action[].fix.helpText` Text to display to the user if this action fails.
- `.spec.action[].fix.helpUrl` A URL to provide to the user in case of an error in this action.
- `.spec.description` is a useful description of the setup, used when listing what's available.
