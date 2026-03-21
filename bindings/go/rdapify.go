// Package rdapify provides Go bindings for the rdapify Rust library.
//
// # Requirements
//
// The rdapify cdylib must be compiled and available.  Point CGO_LDFLAGS at
// the directory containing the built library before building:
//
//	CGO_LDFLAGS="-L/path/to/rdapify/target/release -lrdapify" \
//	  go build ./...
//
// # Example
//
//	import "github.com/rdapify/rdapify-go"
//
//	result, err := rdapify.Domain("example.com")
//	if err != nil {
//	    log.Fatal(err)
//	}
//	fmt.Println(result.Registrar)
package rdapify

/*
#cgo LDFLAGS: -lrdapify
#include "rdapify.h"
#include <stdlib.h>
*/
import "C"
import (
	"encoding/json"
	"errors"
	"unsafe"
)

// ── Response types ────────────────────────────────────────────────────────────

// DomainResponse contains normalised RDAP data for a domain name.
type DomainResponse struct {
	Query      string   `json:"query"`
	LdhName    *string  `json:"ldh_name"`
	Registrar  *string  `json:"registrar"`
	Status     []string `json:"status"`
	ExpiresAt  *string  `json:"expires_at"`
}

// IpResponse contains normalised RDAP data for an IP address.
type IpResponse struct {
	Query   string   `json:"query"`
	Country *string  `json:"country"`
	Name    *string  `json:"name"`
	Status  []string `json:"status"`
}

// AsnResponse contains normalised RDAP data for an ASN.
type AsnResponse struct {
	Query  uint32   `json:"query"`
	Name   *string  `json:"name"`
	Status []string `json:"status"`
}

// NameserverResponse contains normalised RDAP data for a nameserver.
type NameserverResponse struct {
	Query   string   `json:"query"`
	LdhName *string  `json:"ldh_name"`
	Status  []string `json:"status"`
}

// EntityResponse contains normalised RDAP data for an entity.
type EntityResponse struct {
	Query  string   `json:"query"`
	Handle *string  `json:"handle"`
	Roles  []string `json:"roles"`
}

// ── Query functions ───────────────────────────────────────────────────────────

// Domain queries RDAP information for a domain name.
func Domain(domain string) (*DomainResponse, error) {
	cDomain := C.CString(domain)
	defer C.free(unsafe.Pointer(cDomain))

	raw := C.rdapify_domain(cDomain)
	if raw == nil {
		return nil, errors.New("rdapify: domain query failed")
	}
	defer C.rdapify_free_string(raw)

	var result DomainResponse
	if err := json.Unmarshal([]byte(C.GoString(raw)), &result); err != nil {
		return nil, err
	}
	return &result, nil
}

// IP queries RDAP information for an IP address (IPv4 or IPv6).
func IP(ip string) (*IpResponse, error) {
	cIP := C.CString(ip)
	defer C.free(unsafe.Pointer(cIP))

	raw := C.rdapify_ip(cIP)
	if raw == nil {
		return nil, errors.New("rdapify: ip query failed")
	}
	defer C.rdapify_free_string(raw)

	var result IpResponse
	if err := json.Unmarshal([]byte(C.GoString(raw)), &result); err != nil {
		return nil, err
	}
	return &result, nil
}

// ASN queries RDAP information for an Autonomous System Number.
func ASN(asn string) (*AsnResponse, error) {
	cASN := C.CString(asn)
	defer C.free(unsafe.Pointer(cASN))

	raw := C.rdapify_asn(cASN)
	if raw == nil {
		return nil, errors.New("rdapify: asn query failed")
	}
	defer C.rdapify_free_string(raw)

	var result AsnResponse
	if err := json.Unmarshal([]byte(C.GoString(raw)), &result); err != nil {
		return nil, err
	}
	return &result, nil
}

// Nameserver queries RDAP information for a nameserver hostname.
func Nameserver(hostname string) (*NameserverResponse, error) {
	cHostname := C.CString(hostname)
	defer C.free(unsafe.Pointer(cHostname))

	raw := C.rdapify_nameserver(cHostname)
	if raw == nil {
		return nil, errors.New("rdapify: nameserver query failed")
	}
	defer C.rdapify_free_string(raw)

	var result NameserverResponse
	if err := json.Unmarshal([]byte(C.GoString(raw)), &result); err != nil {
		return nil, err
	}
	return &result, nil
}

// Entity queries RDAP information for an entity (contact / registrar).
func Entity(handle, serverURL string) (*EntityResponse, error) {
	cHandle := C.CString(handle)
	defer C.free(unsafe.Pointer(cHandle))

	cURL := C.CString(serverURL)
	defer C.free(unsafe.Pointer(cURL))

	raw := C.rdapify_entity(cHandle, cURL)
	if raw == nil {
		return nil, errors.New("rdapify: entity query failed")
	}
	defer C.rdapify_free_string(raw)

	var result EntityResponse
	if err := json.Unmarshal([]byte(C.GoString(raw)), &result); err != nil {
		return nil, err
	}
	return &result, nil
}
