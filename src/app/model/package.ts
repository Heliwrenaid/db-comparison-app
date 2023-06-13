export interface PackageData {
    basic: BasicPackageData,
    additional: AdditionalPackageData,
    dependencies: PackageDependency[],
    comments: Comment[],
}

export interface BasicPackageData {
    name: string,
    version: string,
    pathToAdditionalData: string,
    votes: number,
    popularity: number,
    description: string,
    maintainer: string,
    lastUpdated: string,
}

export interface AdditionalPackageData {
    gitCloneUrl: string,
    keywords: string | null,
    license: string | null,
    confilcts: string | null,
    provides: string | null,
    submitter: string,
    firstSubmitted: string,
}

export interface PackageDependency {
    group: string,
    packages: string[],
}

export interface Comment {
     header: string,
     content: string,
}
