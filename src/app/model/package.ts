export interface PackageData {
    basic: BasicPackageData,
    additional: AdditionalPackageData,
    dependencies: PackageDependency[],
    comments: Comment[],
}

export interface BasicPackageData {
    name: string,
    version: string,
    path_to_additional_data: string,
    votes: number,
    popularity: number,
    description: string,
    maintainer: string,
    last_updated: string,
}

export interface AdditionalPackageData {
    git_clone_url: string,
    keywords: string | null,
    license: string | null,
    confilcts: string | null,
    provides: string | null,
    submitter: string,
    first_submitted: string,
}

export interface PackageDependency {
    group: string,
    packages: string[],
}

export interface Comment {
     header: string,
     content: string,
}
