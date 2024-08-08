# WARNING
This cli was put together in a few hours, so bug will happen.
Also this cli is currently only tested on Windows.
# Obsidian Vault Templater
This is a small cli for creating obsidian vaults from a template folder.
# Use Case
I found it tedious to configure my plugins and setting, which i use in every vault anyway, over and over again.
So i wrote a small cli for creating a new vault from a template vault on the fly.
# Setup
```cmd
ovt template "path/to/template/vault"
ovt vaults "path/to/vaults/folder"
ovt obsidian "path/to/obsidian/installation"
```
# Examples
```cmd
ovt -n <new vault name>
```