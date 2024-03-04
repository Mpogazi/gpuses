FROM ubuntu:24.04
RUN apt-get update && apt-get install -y openssh-server sudo
RUN mkdir -p /var/run/sshd
# authorize SSH connection with root account
RUN echo 'root:pass_123' | chpasswd
RUN sed -i 's/PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
RUN service ssh start

EXPOSE 22
CMD ["/usr/sbin/sshd", "-D"]