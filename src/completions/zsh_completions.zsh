#compdef git-ignore

_git-ignore() {
  local -a args
  local -a lang_list

  lang_list=("${(@f)$(git ignore --list)}")

  _arguments \
    '1: :->args' \
    '*: :->subcmd'

  case $state in
    (args)
      compadd "$@" $lang_list
      _arguments \
        '-h[Print this help message]' \
        '--help[Print this help message]' \
        '-V[Print version information and exit]' \
        '--version[Print version information and exit]' \
        '--repo[Print gitignore repository path and exit]' \
        '--list[List all available gitignore files]' \
        '-c[Generate completion script for bash, zsh or fish]:completion script:(bash zsh fish)'
      ;;
    (subcmd)
      compadd "$@" $lang_list
      ;;
  esac
}

_git-ignore "$@"
