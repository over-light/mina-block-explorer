suite(["@CI"], "empty table", () => {
  let pages = [
    "/blocks?q-state-hash=3Nfake",
    "/commands/user-commands?query=fake",
    "/commands/internal-commands?q-recipient=B62qfake",
    "/snarks?q-state-hash=3Nfake",
    "/staking-ledgers?query=fake",
    "/next-stakes?query=fake",
  ];

  pages.forEach((page) =>
    it(`on ${page} shows as having zero records`, () => {
      cy.visit(page);
      cy.contains("Showing 0 to 0 of 0 records");
    }),
  );
});
