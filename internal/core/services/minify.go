package services

import (
	"math/rand"

	"github.com/josiahdenton/shrink-my-url/internal/core/domain"
	"github.com/josiahdenton/shrink-my-url/internal/core/ports"
)


const alphanumeric = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456789"
const MINIFY_SIZE = 7 // https://hostname/5rt3csf.com


type MinifyUrlService struct{
    urlRepository ports.UrlRepository
}

func (m *MinifyUrlService) Minify(url domain.MinifyUrl) (string, error) {
	if len(url.Alias) > 0 {

	}

	m.urlRepository.Contains(url.Alias)

	// todo implement
	return "", nil
}

func generateRandString() string {
    b := make([]byte, MINIFY_SIZE) 
	for i := range b {
		b[i] = alphanumeric[rand.Intn(len(alphanumeric))]
	}
	return string(b)
}
