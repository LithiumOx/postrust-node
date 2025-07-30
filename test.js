const { init, getInfo, lookup, lookupBatch } = require("./index.js");

console.log("🧪 Testing postRUST NPM Package");
console.log("================================");

// Initialize the data
console.log("📦 Initializing...");
init();

// Get package info
console.log("📊 Package info:");
console.log(getInfo());
console.log();

// Test single lookup
console.log("🔍 Testing single lookup...");
const result1 = lookup("1234AB", 1);
console.log("Result:", result1);
console.log();

// Test batch lookup
console.log("🔍 Testing batch lookup...");
const queries = [
	["1011VX", 2],
	["8218NE", 28],
	["9999ZZ", 999], // This should not exist
];

const batchResults = lookupBatch(queries);
console.log("Batch results:");
batchResults.forEach((result, i) => {
	console.log(`  Query ${i + 1} (${queries[i][0]}, ${queries[i][1]}):`, result);
});

console.log();
console.log("✅ Test completed!");
console.log();
console.log("🎯 Usage in your Node.js project:");
console.log("");
console.log('const { lookup } = require("@postrust/lookup");');
console.log("");
console.log('const result = lookup("1234AB", 1);');
console.log("if (result) {");
console.log(
	"  console.log(`${result.straat} ${result.huisnummer}, ${result.woonplaats}`);",
);
console.log("}");
