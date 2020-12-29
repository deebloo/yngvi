FROM node:15.5.0 as build

COPY . .

RUN npm ci
RUN npm run build

FROM node:15.5.0

WORKDIR /var/weather

COPY --from=build target target
COPY --from=build package*.json ./

RUN npm ci --only=production

CMD [ "node", "target/main.js" ]