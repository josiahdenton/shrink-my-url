package ports

import "github.com/josiahdenton/shrink-my-url/internal/core/domain"

type MinifierService interface {
	// Take in a string and
	// and check the db to see if it can be minified.
	// MinifyUrl will continue to try until a minified url is 
    // successfully generated. This will error on DB errors
	MinifyUrl(url domain.MinifyUrl) (string, error)
}

type UrlRepository interface {
    // Set will add the key value pair in the UrlRepository
    Set(key string, value string) error
    // Get will retrieve the value associated with this key
    Get(key string) (string, error)
    // Contains will check if this UrlRepository contains the key
    Contains(key string) (bool, error)
}
