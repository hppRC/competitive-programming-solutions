#!/bin/bash

echo "# AtCoder

AtCoderの問題を解いた際のプログラムです。
基本的にはこの中のプログラムをそのまま提出すればジャッジに通ると思います。
解法の記録と研究・忘却阻止用のリポジトリです。

organize.shは整理整頓用のスクリプトファイルです。easy.shはソースコードを楽にgit pushしたいがためのファイルになっています。
"
today=$(date "+%Y%m%d")
echo "----------以下現在のファイル数----------
${today}現在"

for x in *
    do echo "${x}"
    ls -1UR $x | wc -l
done