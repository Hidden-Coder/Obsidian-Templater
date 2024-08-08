# WARNING
The current version of this cli is not yet stable and bugs are to be expected.
Also this cli is currently only tested on Windows.
# Prequisites
- [Obsidian](https://obsidian.md/) installed
# Obsidian Vault Templater
This is a small cli for creating [Obsidian](https://obsidian.md/) vaults from a template [Obsidian](https://obsidian.md/) vault.
# Use Case
I found it tedious to configure my plugins and settings, which i use in every vault anyway, over and over again.
So i wrote a small cli for creating a new vault from a template vault on the fly.
# Setup
```cmd
# Set the template vault
ovt template "path/to/template/vault"
# Set the folder for new vaults to be created in
ovt vaults "path/to/vaults/folder"
# Set the path to the obsidian installation
ovt obsidian "path/to/obsidian/installation"
```
# Examples
```cmd
ovt -n <new vault name>
```
