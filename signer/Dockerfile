FROM node:18-alpine AS builder
WORKDIR /app
COPY . ./
RUN yarn install && yarn build


FROM node:18-alpine AS server
WORKDIR /app
COPY --from=builder ./app/dist ./dist
COPY package* ./
COPY tsconfig* ./
COPY prisma/ ./ 
COPY config/ ./config/
RUN yarn global add @nestjs/cli
RUN yarn install --prod && yarn prisma generate
RUN addgroup -S nonroot && adduser -S nonroot -G nonroot && chown -R nonroot:nonroot /app
EXPOSE 8080
USER nonroot:nonroot
ENTRYPOINT ["yarn", "start"]
