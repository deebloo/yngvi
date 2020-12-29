FROM node:15.5.0-alpine as build

COPY package*.json ./
COPY tsconfig.json ./
COPY src src

RUN npm ci
RUN npm run build

FROM node:15.5.0

WORKDIR /var/weather

COPY --from=build target target
COPY --from=build package*.json ./

RUN npm ci --only=production

ENV NODE_ENV=production

CMD [ "node", "target/main.js" ]