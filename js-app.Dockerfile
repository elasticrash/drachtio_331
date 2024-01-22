FROM node:21

COPY ./package.json ./package.json
COPY ./src ./src

RUN npm install
