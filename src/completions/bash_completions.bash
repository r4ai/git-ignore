_git_ignore_complete() {
  local cur prev opts
  cur="${COMP_WORDS[COMP_CWORD]}"
  prev="${COMP_WORDS[COMP_CWORD - 1]}"

  # Command options
  opts="-h --help -V --version --repo --list -c --completion"

  case "${prev}" in
  git)
    COMPREPLY=($(compgen -W "ignore" -- ${cur}))
    return 0
    ;;
  --completion)
    COMPREPLY=($(compgen -W "bash zsh fish" -- ${cur}))
    return 0
    ;;
  *) ;;
  esac

  if [[ ${cur} == * ]]; then
    local ignores
    ignores=$(git ignore --list 2>/dev/null)
    COMPREPLY=($(compgen -W "${opts} ${ignores}" -- ${cur}))
    return 0
  fi
}

complete -F _git_ignore_complete git
