FROM node:lts-alpine AS base
RUN apk add --no-cache libc6-compat rsync \
  && corepack enable && corepack prepare yarn@4.9.1 --activate
WORKDIR /app

FROM base AS deps
COPY package.json yarn.lock .yarnrc.yml ./
COPY packages ./packages
COPY entrypoint/package.json ./entrypoint/
RUN yarn install --no-immutable

FROM base AS builder
COPY --from=deps /app/node_modules ./node_modules
COPY --from=deps /app/packages ./packages
COPY package.json yarn.lock .yarnrc.yml nx.json tsconfig.base.json tsconfig.paths.json ./
COPY entrypoint ./entrypoint
RUN yarn install --no-immutable \
  && yarn nx build entrypoint --prod

FROM base AS runner
WORKDIR /app
RUN addgroup --system --gid 1001 nodejs \
  && adduser --system --uid 1001 nextjs
COPY --from=builder --chown=nextjs:nodejs /app/entrypoint/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/entrypoint/.next/static ./entrypoint/.next/static
COPY --from=builder --chown=nextjs:nodejs /app/entrypoint/public ./entrypoint/public
USER nextjs
CMD ["node", "entrypoint/server.js"]
