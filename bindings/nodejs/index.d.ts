// TypeScript type definitions for @rdapify/core
// Auto-generated — do not edit manually.

// ── Common types ──────────────────────────────────────────────────────────────

export interface ResponseMeta {
  source: string;
  queriedAt: string;
  cached: boolean;
}

export type RdapStatus =
  | "validated"
  | "renew prohibited"
  | "update prohibited"
  | "transfer prohibited"
  | "delete prohibited"
  | "proxy"
  | "private"
  | "removed"
  | "obscured"
  | "associated"
  | "active"
  | "inactive"
  | "locked"
  | "pending create"
  | "pending renew"
  | "pending transfer"
  | "pending update"
  | "pending delete"
  | string;

export type RdapRole =
  | "registrant"
  | "technical"
  | "administrative"
  | "abuse"
  | "billing"
  | "registrar"
  | "reseller"
  | "sponsor"
  | "proxy"
  | "notifications"
  | "noc"
  | string;

export interface RdapEvent {
  eventAction: string;
  eventDate: string;
  eventActor?: string;
}

export interface RdapLink {
  href: string;
  rel?: string;
  type?: string;
  title?: string;
}

export interface RdapRemark {
  title?: string;
  type?: string;
  description: string[];
  links?: RdapLink[];
}

export interface RdapEntity {
  handle?: string;
  vcardArray?: unknown;
  roles: RdapRole[];
  events: RdapEvent[];
  links: RdapLink[];
  remarks: RdapRemark[];
  entities: RdapEntity[];
}

// ── Response types ────────────────────────────────────────────────────────────

export interface RegistrarSummary {
  name?: string;
  handle?: string;
  url?: string;
  abuseEmail?: string;
  abusePhone?: string;
}

export interface DomainResponse {
  query: string;
  ldhName?: string;
  unicodeName?: string;
  handle?: string;
  status: RdapStatus[];
  nameservers: string[];
  registrar?: RegistrarSummary;
  entities: RdapEntity[];
  events: RdapEvent[];
  links: RdapLink[];
  remarks: RdapRemark[];
  meta: ResponseMeta;
}

export interface IpResponse {
  query: string;
  handle?: string;
  startAddress?: string;
  endAddress?: string;
  ipVersion?: "v4" | "v6";
  name?: string;
  allocationType?: string;
  country?: string;
  parentHandle?: string;
  status: RdapStatus[];
  entities: RdapEntity[];
  events: RdapEvent[];
  links: RdapLink[];
  remarks: RdapRemark[];
  meta: ResponseMeta;
}

export interface AsnResponse {
  query: number;
  handle?: string;
  startAutnum?: number;
  endAutnum?: number;
  name?: string;
  autnumType?: string;
  country?: string;
  status: RdapStatus[];
  entities: RdapEntity[];
  events: RdapEvent[];
  links: RdapLink[];
  remarks: RdapRemark[];
  meta: ResponseMeta;
}

export interface NameserverIpAddresses {
  v4: string[];
  v6: string[];
}

export interface NameserverResponse {
  query: string;
  handle?: string;
  ldhName?: string;
  unicodeName?: string;
  ipAddresses: NameserverIpAddresses;
  status: RdapStatus[];
  entities: RdapEntity[];
  events: RdapEvent[];
  links: RdapLink[];
  remarks: RdapRemark[];
  meta: ResponseMeta;
}

export interface EntityResponse {
  query: string;
  handle?: string;
  vcardArray?: unknown;
  roles: RdapRole[];
  status: RdapStatus[];
  entities: RdapEntity[];
  events: RdapEvent[];
  links: RdapLink[];
  remarks: RdapRemark[];
  meta: ResponseMeta;
}

// ── Exported functions ────────────────────────────────────────────────────────

/**
 * Query RDAP information for a domain name.
 * @param domain - Domain name (e.g. "example.com"). Unicode IDNs are supported.
 */
export declare function domain(domain: string): Promise<DomainResponse>;

/**
 * Query RDAP information for an IP address.
 * @param ip - IPv4 or IPv6 address (e.g. "8.8.8.8")
 */
export declare function ip(ip: string): Promise<IpResponse>;

/**
 * Query RDAP information for an Autonomous System Number.
 * @param asn - ASN as string: "15169" or "AS15169"
 */
export declare function asn(asn: string): Promise<AsnResponse>;

/**
 * Query RDAP information for a nameserver.
 * @param hostname - Nameserver hostname (e.g. "ns1.google.com")
 */
export declare function nameserver(hostname: string): Promise<NameserverResponse>;

/**
 * Query RDAP information for an entity (contact / registrar).
 * @param handle    - Entity handle (e.g. "ARIN-HN-1")
 * @param serverUrl - RDAP server base URL (required — no global bootstrap for entities)
 */
export declare function entity(handle: string, serverUrl: string): Promise<EntityResponse>;
