import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@CI"], "block spotlight", () => {
  [
    {
      origin: "/blocks",
      selector: 'a[href^="/blocks/"]:not(a[href^="/blocks/account"])',
    },
    {
      origin: "/blocks",
      selector: 'a[href^="/blocks/"]:not(a[href^="/blocks/account"])',
    },
    {
      origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
      selector: 'a[href^="/blocks/"]:not(a[href^="/blocks/account"])',
    },
    {
      origin: `/commands/internal-commands`,
      selector: 'a[href^="/blocks/"]',
    },
  ].forEach(({ origin, selector }) =>
    it(`is navigated to from ${origin}`, () => {
      cy.visit(origin);
      cy.wait(1000);
      cy.get(selector, { timeout: 10000 }).first().click({ force: true });
      cy.wait(1000);
      cy.url().should("include", "/blocks/", { timeout: 10000 });
    }),
  );
});
