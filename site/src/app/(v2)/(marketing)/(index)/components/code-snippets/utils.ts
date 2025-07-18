import {
	faFilePen,
	faRobot,
	faMessage,
	faDatabase,
	faGaugeHigh,
	faWaveSine,
	faGamepad,
	faRotate,
	faBuilding,
	faCode,
	faNpm,
	faFlask,
	faBeaker,
	faReact,
	faBracketsCurly,
	faJs,
	faFile,
} from "@rivet-gg/icons";
import { examples } from "@/data/examples/examples";
import JSZip from "jszip";

export const EXAMPLE_ICON_MAP: Record<string, any> = {
	"ai-agent": faRobot,
	"chat-room": faMessage,
	crdt: faFilePen,
	database: faDatabase,
	rate: faGaugeHigh,
	stream: faWaveSine,
	game: faGamepad,
	sync: faRotate,
	tenant: faBuilding,
};

// Shared functionality for example actions
export const createExampleActions = (exampleId: string, exampleFiles?: Record<string, string>) => {
	const handleOpenGithub = () => {
		window.open(
			`https://github.com/rivet-gg/rivetkit/tree/main/examples/${exampleId}`,
			"_blank",
		);
	};

	const handleOpenStackBlitz = () => {
		const stackBlitzUrl = `https://stackblitz.com/github/rivet-gg/rivetkit/tree/main/examples/${exampleId}`;
		window.open(stackBlitzUrl, "_blank");
	};

	const handleDownloadZip = async () => {
		if (!exampleFiles) {
			const exampleData = examples.find((ex) => ex.id === exampleId);
			if (!exampleData) return;
			exampleFiles = exampleData.files;
		}

		const zip = new JSZip();

		Object.entries(exampleFiles).forEach(([filePath, fileContent]) => {
			zip.file(filePath, fileContent);
		});

		const zipBlob = await zip.generateAsync({ type: "blob" });
		const url = URL.createObjectURL(zipBlob);
		const a = document.createElement("a");
		a.href = url;
		a.download = `${exampleId}.zip`;
		document.body.appendChild(a);
		a.click();
		document.body.removeChild(a);
		URL.revokeObjectURL(url);
	};

	return {
		handleOpenGithub,
		handleOpenStackBlitz,
		handleDownloadZip,
	};
};

export function getFileIcon(fileName: string) {
	// Check for specific file names first
	if (fileName === "package.json") return faNpm;
	if (fileName === "tsconfig.json") return faBracketsCurly;

	// Check for test files first (before other extensions)
	if (fileName.includes(".test.") || fileName.includes(".spec."))
		return faFlask;

	// Check for file extensions
	if (fileName.endsWith(".tsx")) return faReact;
	if (fileName.endsWith(".ts")) return faJs;
	if (fileName.endsWith(".js") || fileName.endsWith(".jsx")) return faCode;
	if (fileName.endsWith(".json")) return faBracketsCurly;

	// Default file icon
	return faFile;
}