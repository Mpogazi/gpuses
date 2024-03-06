FROM ubuntu:24.04

# Set environment variables
ENV USER_NAME kowry-user
ENV USER_HOME /home/${USER_NAME}

# Update package lists and install necessary packages
RUN apt-get update \
    && apt-get install -y openssh-server sudo \
    && rm -rf /var/lib/apt/lists/*

# Create sshd file
RUN mkdir -p /var/run/sshd

# Create a base user
RUN useradd -m -d /home/${USER_NAME} -s /bin/bash ${USER_NAME} 

# Set up SSH to use key authentication and disable password authentication
RUN sed -i 's/#\?PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config && \
    sed -i 's/#\?PubkeyAuthentication no/PubkeyAuthentication yes/' /etc/ssh/sshd_config

# Ensure that the SSHD config file allows key-based authentication and denies empty passwords
RUN echo "PasswordAuthentication no" >> /etc/ssh/sshd_config && \
    echo "PubkeyAuthentication yes" >> /etc/ssh/sshd_config && \
    echo "PermitEmptyPasswords no" >> /etc/ssh/sshd_config

# path to the ssh file
ARG SSH_KEY_PATH

# Copy the ssh key file
COPY ${SSH_KEY_PATH} ${USER_HOME}/.ssh/authorized_keys

RUN chown ${USER_NAME}:${USER_NAME} ${USER_HOME}/.ssh/authorized_keys && \
    chmod 600 ${USER_HOME}/.ssh/authorized_keys && \
    chmod 700 ${USER_HOME}/.ssh

# Generate SSH host keys
RUN sudo ssh-keygen -A

# Start the ssh server
RUN service ssh start

# Expose SSH port
EXPOSE 22
# Start SSH server
CMD ["/usr/sbin/sshd", "-D"]