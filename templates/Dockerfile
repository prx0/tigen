FROM docker.io/archlinux/archlinux:latest
ENV NAME="{{ name }}" VERSION="{{ version }}"
LABEL com.github.containers.toolbox="true" \
  name="$NAME" \
  version="$VERSION"
RUN {{ run_layer }} \
  && echo "%wheel ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/toolbox
CMD ["bash"]