#!/usr/bin/env fish
# -*-  mode:fish; tab-width:2; encoding:utf-8 -*-
#
# reference:
# https://github.com/fish-shell/fish-shell/blob/85deb76c5f42c80fff0a3b63def4e1e176daf002/share/completions/git.fish#L544

complete -c git -n __fish_use_subcommand -xa ignore -d 'Generate gitignore'
complete -c git -n '__fish_git_using_command ignore' -xa "(git ignore --list)"
complete -c git -n '__fish_git_using_command ignore' -s h -l help -d 'Print this help message'
complete -c git -n '__fish_git_using_command ignore' -s V -l version -d 'Print version information and exit'
complete -c git -n '__fish_git_using_command ignore' -l repo -d 'Print gitignore repository path and exit'
complete -c git -n '__fish_git_using_command ignore' -l list -d 'List all available gitignore files'
complete -c git -n '__fish_git_using_command ignore' -s c -l completion -xa bash -d 'Generate completion script for bash'
complete -c git -n '__fish_git_using_command ignore' -s c -l completion -xa zsh -d 'Generate completion script for zsh'
complete -c git -n '__fish_git_using_command ignore' -s c -l completion -xa fish -d 'Generate completion script for fish'
