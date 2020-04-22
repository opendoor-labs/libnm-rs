FROM archlinux:latest

RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm gcc
RUN pacman -S --noconfirm base-devel
RUN pacman -S --noconfirm rust
RUN pacman -S --noconfirm libnm

WORKDIR /app
COPY . .

# Run with `docker build -it --rm $(pwd):/app thiscontainer` to mount your local directory
CMD ["cargo", "build", "--examples"]
