local claims = {
  email_verified: false,
} + std.extVar('claims');
{
  identity: {
    traits: {
      [if 'email' in claims then 'email' else null]: claims.email,
      [if 'name' in claims then 'username' else null]: std.strReplace(claims.name, ' ', '_'),
    },
  },
}
