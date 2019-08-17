FROM ubuntu:latest

ENV DEBIAN_FRONTEND noninteractive

RUN apt update
RUN apt -y upgrade
RUN apt -y install nmap libsqlite3-0
RUN apt -y autoremove
RUN apt -y clean


ADD assets/log4rs.yml /etc/axis/
ADD LICENSE README.md /usr/share/axis/
ADD node_modules /usr/share/axis/node_modules
ADD target/release/axis /usr/bin/axis
ADD var /var/axis

CMD ["/usr/bin/axis"]
