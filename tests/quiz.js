const anchor = require('@project-serum/anchor')
const {SystemProgram} = anchor.web3;

const assert = require('assert')

const baseAccount = anchor.web3.Keypair.generate()

describe("quiz", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Quiz;

  it("Is initialized!", async () => {
    // Add your test here.

    let questions = [ { name: 'hey', rightOption: 0, options: [ 'Wassup' ] }, { name: 'hey bro', rightOption: 1, options: [ 'Wassup', 'cool', 'yo' ] }, { name: 'how are u doing', rightOption: 2, options: [ 'Wassup', 'cool', 'yo', 'sup' ] }  ]

    let quizName = "Test"

    let tx = await program.rpc.initialize({
      accounts: {
        quiz: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [baseAccount]
    });

    console.log("Your transaction signature", tx);

    tx = await program.rpc.addQuiz(quizName, questions, {
      accounts: {
        quiz: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      }
    })

    console.log(tx);

    const vals = await program.account.quiz.fetch(baseAccount.publicKey);
    console.log(vals.totalQuiz)
    console.log(vals.quizList)
    console.log(vals.quizList[0].questions)

    const scores = [0, 1, 0];

    tx = await program.rpc.calculateScore(0, new Buffer(scores),{
      accounts: {
        quiz: baseAccount.publicKey,
        user: provider.wallet.publicKey
      }
    });

    const val = await program.account.quiz.fetch(baseAccount.publicKey);
    console.log(val.quizList[0].scores)


    assert.ok(vals.totalQuiz.eq(new anchor.BN(1)))
  });
});
