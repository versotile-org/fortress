_Note_: **THIS REPOSITORY IS A MIRROR AND WILL ONLY RECEIVES POINT UPDATES.**

# FORTRESS

At a time when cyber threats are rampant and data breaches can have catastrophic consequences,
the focus on robust software security has never been more important.
The need to protect private data is not just a compliance issue, but a fundamental aspect of digital trust.
This application proposes to develop a modular security component with a similar approach to
the IOTA Foundation's Project `Stronghold`.

**FORTRESS** is a software enclave similar to hardware keys such as yubikey and similar products.
The main idea is to store sensitive data that is not directly accessible to the user. Rather,
any cryptographic operation required, such as signing, encrypting, decrypting, etc.,
takes place inside a so-called vault: a fraction of the system memory that remains highly guarded
and encrypted with the latest state-of-the-art encryption, preferably `xchacha20-poly1305`.

The goal of this project is to build a lightweight system for the secure storage of e.g. identity data.
In this context, we assume that identity data is implemented using public key cryptography such as `DID`.

The system will consist of at least the following models, which will be described in more detail.

## Vault

The **Vault** is the centerpiece of our quest to redefine data security paradigms.
Designed to operate in a virtualized environment, the **Vault** provides an impenetrable layer of isolation,
separating its operations from ancillary system processes.
This strategic isolation is further reinforced by the use of a security-hardened lightweight kernel
within the virtual machine environment. One of the possible options is the [seL4] microkernel,
as it has an unparalleled track record in delivering robust, mathematically proven security properties,
as well as specialized memory access capabilities that are essential to our advanced security model.

At the heart of the **Vault**'s operational doctrine is the management of data entry, orchestrated through
the use of cryptographically secure hashed keys. This mechanism ensures that each data entry is
uniquely identifiable but unpredictable, thereby reinforcing data integrity and confidentiality.
To reinforce the sanctity of data at rest, the **Vault** uses state-of-the-art symmetric encryption
algorithms, with a preference for `xchacha20-poly1305` or `AES-256`. These algorithms are renowned for
their cryptographic strength and efficiency, making them ideal for our high-security component.

The **Vault**'s architecture incorporates an innovative approach to memory storage,
utilizing non-contiguous memory segments. This design philosophy is synergised with the Boojum scheme,
a technique that dynamically rotates memory fragments.
Such a methodology not only complicates unauthorized data reconstruction, but also provides a safeguard
against accidental core dumps, reducing the risk of accidental data exposure.
The **Vault** can quickly reconstruct the original data from its rotated fragments.

## Data Exchange Format

Sensitive data shall also be able to be persisted. The basic data format shall be a common human-readable
format like `JSON`, `Toml`, or `Yaml`. While stored the data will be encrypted with authenticated
symmetric encryption algorithms like `xchacha20-poly1305`.

## Compatibility / Hardware Support

Within the **FORTRESS** Project, a fundamental pillar of our architectural philosophy is the seamless
integration and compatibility with established cryptographic APIs such as `PKCS#11 (also known as Cryptoki)`.
This strategic decision is designed to facilitate the use of a wide range of security hardware sticks,
ensuring that **FORTRESS** can be easily adopted into a wide range of existing security infrastructures
without the need for extensive modification.
In addition, the project is committed to embracing advances in authentication technologies,
with a particular focus on the `FIDO 2` standard. This inclusion not only simplifies the process of
two-factor authentication for access to common websites, but also heralds the adoption of cutting-edge
identity standards such as passkeys, which are poised to revolutionize authentication practices by
eliminating the need for traditional passwords.

Crucially, **FORTRESS** is committed to aligning with and supporting identity regulations,
in particular `eIDAS 2.0`. This commitment ensures that **FORTRESS** will be at the forefront of
facilitating secure, cross-border digital identities within the European Union, thereby increasing trust
and security in electronic transactions across member states. By integrating support for `eIDAS 2.0`,
**FORTRESS** positions itself as a versatile and future-proof solution capable of meeting the evolving
needs for digital identity verification and authentication in a manner that is both secure and easy to use.
This strategic direction underscores our commitment to advancing the state of digital security and identity
management, ensuring that **FORTRESS** remains compatible with and relevant to the latest standards and
practices in the field.

[seL4]: https://sel4.systems/
