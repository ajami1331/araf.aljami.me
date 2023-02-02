FROM node:latest as builder

WORKDIR /app/

COPY src/ src/
COPY js/ js/
COPY dist/ dist/
COPY package.json .
COPY package-lock.json .

RUN npm install
RUN npm start

FROM busybox:latest

WORKDIR /root

COPY --from=builder /app/dist dist

CMD ["busybox", "httpd", "-f", "-v", "-p", "3000", "-h", "dist"]