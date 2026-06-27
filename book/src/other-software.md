# Zemacs mode in other software

Zemacs' keymap and interaction model ([Using Zemacs](./usage.md)) is easier to adopt if it can be used consistently in many editing contexts. Yet, certain use cases cannot easily be addressed directly in Zemacs. Similar to vim, this leads to the creation of "Zemacs mode" in various other software products, allowing Zemacs-style editing for a greater variety of use cases.

"Zemacs mode" is frequently still in early stages or missing entirely. For such cases, we also link to relevant bugs or discussions.

## Other editors

| Editor | Plugin or feature providing Zemacs editing | Comments
| --- | --- | --- |
| [Vim](https://www.vim.org/) | [zemacs.vim](https://github.com/chtenb/zemacs.vim) config |
| [IntelliJ IDEA](https://www.jetbrains.com/idea/) / [Android Studio](https://developer.android.com/studio)| [IdeaVim](https://plugins.jetbrains.com/plugin/164-ideavim) plugin + [zemacs.idea.vim](https://github.com/chtenb/zemacs.vim) config | Minimum recommended version is IdeaVim 2.19.0.
| [Visual Studio](https://visualstudio.microsoft.com/) | [VsVim](https://marketplace.visualstudio.com/items?itemName=JaredParMSFT.VsVim) plugin + [zemacs.vs.vim](https://github.com/chtenb/zemacs.vim) config | 
| [Visual Studio Code](https://code.visualstudio.com/) | [Dance](https://marketplace.visualstudio.com/items?itemName=gregoire.dance) extension, or its [Zemacs fork](https://marketplace.visualstudio.com/items?itemName=kend.dancehelixkey) | The Zemacs fork has diverged. You can also use the original Dance and tweak its keybindings directly (try [this config](https://github.com/71/dance/issues/299#issuecomment-1655509531)).
| [Visual Studio Code](https://code.visualstudio.com/) | [Zemacs for VS Code](https://marketplace.visualstudio.com/items?itemName=jasew.vscode-zemacs-emulation) extension|
| [Zed](https://zed.dev/) | native via keybindings ([Bug](https://github.com/zed-industries/zed/issues/4642)) |
| [CodeMirror](https://codemirror.net/) | [codemirror-zemacs](https://gitlab.com/_rvidal/codemirror-zemacs) |
| [Lite XL](https://lite-xl.com/) | [lite-modal-hx](https://codeberg.org/Mandarancio/lite-modal-hx) |
| [Lapce](https://lap.dev/lapce/) | | Requested: https://github.com/lapce/lapce/issues/281 |


## Shells

| Shell | Plugin or feature providing Zemacs editing 
| --- | --- 
| Fish | [Feature Request](https://github.com/fish-shell/fish-shell/issues/7748) 
| Fish | [fish-zemacs](https://github.com/sshilovsky/fish-zemacs/tree/main) 
| Zsh | [zemacs-zsh](https://github.com/john-h-k/zemacs-zsh) or [zsh-zemacs-mode](https://github.com/Multirious/zsh-zemacs-mode)
| Nushell | [Feature Request](https://github.com/nushell/reedline/issues/639) 

## Other software

| Software | Plugin or feature providing Zemacs editing. | Comments
| --- | --- | --- |
| [Obsidian](https://obsidian.md/) | [Obsidian-Zemacs](https://github.com/Sinono3/obsidian-zemacs) | Uses `codemirror-zemacs` listed above.
