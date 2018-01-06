FROM ubuntu:latest

RUN apt-get update 
RUN apt-get install -y postgresql

WORKDIR /code

ADD target/packaged.zip /code

ENV ROCKET_ENV production
EXPOSE 80

CMD [ "/code/website-server" ]