type Belnapian = "N" | "F" | "T" | "B";

function b_and(a: Belnapian, b: Belnapian): Belnapian {
	if (a === "F" || b === "F") return "F";
	if ((a === "N" && b === "B") || (a === "B" && b === "N")) return "F";
	if (a === "N" || b === "N") return "N";
	if (a === "B" || b === "B") return "B";
	if (a === "T" && b === "T") return "T";

	throw new Error("Unreachable");
}

function b_or(a: Belnapian, b: Belnapian): Belnapian {
	if (a === "T" || b === "T") return "T";
	if ((a === "B" && b === "N") || (a === "N" && b === "B")) return "T";
	if (a === "B" || b === "B") return "B";
	if (a === "N" || b === "N") return "N";
	if (a === "F" && b === "F") return "F";

	throw new Error("Unreachable");
}

function superposition(a: Belnapian, b: Belnapian): Belnapian {
	if (a === "B" || b === "B") return "B";
	if (a === "N") return b;
	if (b === "N") return a;
	if (a === "T" || b === "T") return "T";
	if (a === "F" || b === "F") return "F";
	if ((a === "F" && b === "T") || (a === "T" && b === "F")) return "B";

	throw new Error("Unreachable");
}

function annihilation(a: Belnapian, b: Belnapian): Belnapian {
	if (a === "N" || b === "N") return "N";
	if (a === "B") return b;
	if (b === "B") return a;
	if (a === "T" && b === "F") return "N";
	if (a === "F" && b === "T") return "N";
	if (a === "T" && b === "T") return "T";
	if (a === "F" && b === "F") return "F";

	throw new Error("Unreachable");
}

const NF__ = new Set<Belnapian>(["N", "F"]);
const N_T_ = new Set<Belnapian>(["N", "T"]);
const _FT_ = new Set<Belnapian>(["F", "T"]);
const NFT_ = new Set<Belnapian>(["N", "F", "T"]);
const N__B = new Set<Belnapian>(["N", "B"]);
const _F_B = new Set<Belnapian>(["F", "B"]);
const NF_B = new Set<Belnapian>(["N", "F", "B"]);
const __TB = new Set<Belnapian>(["T", "B"]);
const N_TB = new Set<Belnapian>(["N", "T", "B"]);
const _FTB = new Set<Belnapian>(["F", "T", "B"]);
const NFTB = new Set<Belnapian>(["N", "F", "T", "B"]);

const unknown_values = [
	NF__,
	N_T_,
	_FT_,
	NFT_,
	N__B,
	_F_B,
	NF_B,
	__TB,
	N_TB,
	_FTB,
	NFTB,
];

const belnap_values = ["N", "F", "T", "B"] as const;

function get_set_name(set: Set<Belnapian>): string {
	let name = "";

	name += set.has("N") ? "N" : "_";
	name += set.has("F") ? "F" : "_";
	name += set.has("T") ? "T" : "_";
	name += set.has("B") ? "B" : "_";

	return name;
}

function get_result_name(result: Set<Belnapian>): string {
	const _name = get_set_name(result);

	if (_name === "N___") return "EBelnapian::Known(Belnapian::Neither)";
	if (_name === "_F__") return "EBelnapian::Known(Belnapian::False)";
	if (_name === "__T_") return "EBelnapian::Known(Belnapian::True)";
	if (_name === "___B") return "EBelnapian::Known(Belnapian::Both)";
	if (_name === "____") return "EBelnapian::Unknown(FAIL_MISERABLY)";

	return `EBelnapian::Unknown(Unknown::${_name})`;
}

function print_truthsets_optable_line(
	a: Set<Belnapian>,
	b: Set<Belnapian>,
	result: Set<Belnapian>,
): void {
	console.log(
		`(Unknown::${get_set_name(a)}, Unknown::${get_set_name(b)}) => ${get_result_name(result)},`,
	);
}

function compute_unknown_and_table(): void {
	for (const u of unknown_values) {
		for (const v of unknown_values) {
			const result = new Set<Belnapian>();

			for (const a of u) {
				for (const b of v) {
					result.add(b_and(a, b));
				}
			}

			print_truthsets_optable_line(u, v, result);
		}
	}
}

// compute_unknown_and_table();

function compute_unknown_or_table(): void {
	for (const u of unknown_values) {
		for (const v of unknown_values) {
			const result = new Set<Belnapian>();

			for (const a of u) {
				for (const b of v) {
					result.add(b_or(a, b));
				}
			}

			print_truthsets_optable_line(u, v, result);
		}
	}
}

// compute_unknown_or_table();

function compute_unknown_superposition_table(): void {
	for (const u of unknown_values) {
		for (const v of unknown_values) {
			const result = new Set<Belnapian>();

			for (const a of u) {
				for (const b of v) {
					result.add(superposition(a, b));
				}
			}

			print_truthsets_optable_line(u, v, result);
		}
	}
}

// compute_unknown_superposition_table();

function compute_unknown_annihilation_table(): void {
	for (const u of unknown_values) {
		for (const v of unknown_values) {
			const result = new Set<Belnapian>();

			for (const a of u) {
				for (const b of v) {
					result.add(annihilation(a, b));
				}
			}

			print_truthsets_optable_line(u, v, result);
		}
	}
}

// compute_unknown_annihilation_table();

function get_belnapian_name(b: Belnapian): string {
	switch (b) {
		case "N":
			return "Belnapian::Neither";
		case "F":
			return "Belnapian::False";
		case "T":
			return "Belnapian::True";
		case "B":
			return "Belnapian::Both";
	}
}

function print_truthsets_mixed_line(
	a: Belnapian,
	b: Set<Belnapian>,
	result: Set<Belnapian>,
): void {
	console.log(
		`(${get_belnapian_name(a)}, Unknown::${get_set_name(b)}) => ${get_result_name(result)},`,
	);
}

function compute_and_mixed_table(): void {
	for (const a of belnap_values) {
		for (const u of unknown_values) {
			const result = new Set<Belnapian>();

			for (const b of u) {
				result.add(b_and(a, b));
			}

			print_truthsets_mixed_line(a, u, result);
		}
	}
}

// compute_and_mixed_table();

function compute_or_mixed_table(): void {
	for (const a of belnap_values) {
		for (const u of unknown_values) {
			const result = new Set<Belnapian>();

			for (const b of u) {
				result.add(b_or(a, b));
			}

			print_truthsets_mixed_line(a, u, result);
		}
	}
}

// compute_or_mixed_table();

function compute_superposition_mixed_table(): void {
	for (const a of belnap_values) {
		for (const u of unknown_values) {
			const result = new Set<Belnapian>();

			for (const b of u) {
				result.add(superposition(a, b));
			}

			print_truthsets_mixed_line(a, u, result);
		}
	}
}

// compute_superposition_mixed_table();

function compute_annihilation_mixed_table(): void {
	for (const a of belnap_values) {
		for (const u of unknown_values) {
			const result = new Set<Belnapian>();

			for (const b of u) {
				result.add(annihilation(a, b));
			}

			print_truthsets_mixed_line(a, u, result);
		}
	}
}

compute_annihilation_mixed_table();
