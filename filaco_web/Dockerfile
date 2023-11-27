FROM node:lts-bookworm-slim as builder

WORKDIR /app

COPY package.json .
COPY package-lock.json .

RUN npm ci

COPY . .

RUN npm run build
RUN npm prune --omit=dev

FROM node:lts-bookworm-slim as final

WORKDIR /app

COPY --from=builder /app/build build/
COPY --from=builder /app/node_modules node_modules/

COPY package.json .

CMD [ "node", "build" ]
