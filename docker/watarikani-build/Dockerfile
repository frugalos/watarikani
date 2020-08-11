##
## watarikaniビルド用のベースイメージ
##
FROM centos:centos7.6.1810

##
## yumパッケージ群をインストール
##
RUN yum -y install gcc automake libtool vim git make patch

##
## Rustをインストール
##
RUN curl https://sh.rustup.rs -sSf > rustup.sh && sh rustup.sh -y
ENV PATH $PATH:/root/.cargo/bin

##
## watarikaniをビルド
##
## ビルド対象の git のブランチ/コミットIDを指定する
ARG git_ref=master
RUN git clone https://github.com/frugalos/watarikani.git && cd watarikani && git checkout $git_ref
RUN cd watarikani && cargo build --release --all
