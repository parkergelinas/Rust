/**
 * The state of a greeting account managed by the hello world program
 */
class GreetingAccount {
  counter = 0;
  constructor(fields: { counter: number } | undefined = undefined) {
    if (fields) {
      this.counter = fields.counter;
    }
  }
}

const greeting = borsh.deserialize(
  GreetingSchema,
  GreetingAccount,
  accountInfo.data
);
console.log(
  greetedPubkey.toBase58(),
  "has been greeted",
  greeting.counter,
  "time(s)"
);
