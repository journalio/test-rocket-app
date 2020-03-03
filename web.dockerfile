FROM node:alpine AS builder
WORKDIR /usr/src/test-rocket-app
COPY package.json yarn.lock ./
RUN yarn --frozen-lockfile
COPY . ./
RUN yarn build:prod

FROM nginx:alpine
COPY --from=builder /usr/src/test-rocket-app/public /usr/share/nginx/html
CMD ["nginx", "-g", "daemon off;"]