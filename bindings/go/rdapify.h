/**
 * rdapify-go C ABI header
 *
 * This header declares the C-compatible functions exported by the rdapify
 * cdylib.  Link against librdapify.so / rdapify.dll / librdapify.dylib.
 *
 * Memory ownership:
 *  - All returned *char strings must be freed with rdapify_free_string().
 *  - All other return values are plain integers (no allocation).
 */

#ifndef RDAPIFY_H
#define RDAPIFY_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

/**
 * Query RDAP information for a domain name.
 *
 * @param domain  NUL-terminated ASCII/Unicode domain name.
 * @return        JSON string (caller must call rdapify_free_string).
 *                Returns NULL on error.
 */
char *rdapify_domain(const char *domain);

/**
 * Query RDAP information for an IP address (IPv4 or IPv6).
 *
 * @param ip  NUL-terminated IP address string.
 * @return    JSON string (caller must call rdapify_free_string).
 *            Returns NULL on error.
 */
char *rdapify_ip(const char *ip);

/**
 * Query RDAP information for an Autonomous System Number.
 *
 * @param asn  NUL-terminated ASN string (e.g., "AS15169" or "15169").
 * @return     JSON string (caller must call rdapify_free_string).
 *             Returns NULL on error.
 */
char *rdapify_asn(const char *asn);

/**
 * Query RDAP information for a nameserver hostname.
 *
 * @param hostname  NUL-terminated nameserver hostname.
 * @return          JSON string (caller must call rdapify_free_string).
 *                  Returns NULL on error.
 */
char *rdapify_nameserver(const char *hostname);

/**
 * Query RDAP information for an entity (contact / registrar).
 *
 * @param handle      NUL-terminated entity handle.
 * @param server_url  NUL-terminated base RDAP server URL.
 * @return            JSON string (caller must call rdapify_free_string).
 *                    Returns NULL on error.
 */
char *rdapify_entity(const char *handle, const char *server_url);

/**
 * Free a string returned by any rdapify_* function.
 *
 * @param s  Pointer previously returned by an rdapify_* call.
 */
void rdapify_free_string(char *s);

#ifdef __cplusplus
}
#endif

#endif /* RDAPIFY_H */
