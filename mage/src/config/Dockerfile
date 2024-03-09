FROM ubuntu:24.04

# Set environment variables
ENV USER_NAME kowry-user
ENV USER_HOME /home/${USER_NAME}

# Update package lists and install necessary packages
RUN apt-get update \
    && apt-get install -y openssh-server sudo systemd

# Create a base user
RUN useradd -m -d /home/${USER_NAME} -s /bin/bash ${USER_NAME} 

# Setup SSH for kowry-user
RUN mkdir -p /home/${USER_NAME}/.ssh && \
    chmod 700 /home/${USER_NAME}/.ssh

# Ensure that the SSHD config file allows key-based authentication and denies empty passwords
RUN echo "PasswordAuthentication no" >> /etc/ssh/sshd_config && \
    echo "PubkeyAuthentication yes" >> /etc/ssh/sshd_config && \
    echo "AuthorizedKeysFile /home/${USER_NAME}/.ssh/authorized_keys" >> /etc/ssh/sshd_config

# path to the ssh file
ARG SSH_KEY_PATH

# Copy the ssh key file
COPY ${SSH_KEY_PATH} ${USER_HOME}/.ssh/authorized_keys

RUN chown -R ${USER_NAME}:${USER_NAME} ${USER_HOME}/.ssh && \
    chmod 600 ${USER_HOME}/.ssh/authorized_keys && \
    chmod 700 ${USER_HOME}/.ssh

# Expose SSH port
EXPOSE 22
# Start SSH server
CMD ["usr/sbin/sshd", "-D", "-e", "-f", "/etc/ssh/sshd_config"]