package main

/*
// For reducing the file size of binding file at rust size.
// just declear free function, insteading of including stdlib.h.
void free (void* ptr);
*/
import "C"
import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
	"os"
	"unsafe"
)

type MinioOption struct {
	Endpoint        string
	PublicEndpoint  string
	UseSSL          bool
	AccessKeyID     string
	SecretAccessKey string
	bucketName      string
}

const (
	defaultMinioEndpoint   = "localhost:9000"
	defaultMinioBucketName = "md-tools"
	minioBucketLocation    = ""
)

func FromEnv() (*MinioOption, error) {
	var opt = MinioOption{Endpoint: defaultMinioEndpoint, UseSSL: false, bucketName: defaultMinioBucketName}

	if endpoint := os.Getenv("MINIO_ENDPOINT"); endpoint != "" {
		opt.Endpoint = endpoint
	}
	opt.PublicEndpoint = os.Getenv("PRESIGNED_MINIO_ENDPOINT")
	if opt.PublicEndpoint == "" {
		opt.PublicEndpoint = "http://" + opt.Endpoint
	}

	//publicEndpointUrl, err := url.Parse(PublicEndpoint)
	//if err != nil {
	//	return opt, err
	//}

	if useSSL := os.Getenv("MINIO_SECURE_SSL"); useSSL != "" {
		opt.UseSSL = true
	}

	if bucketName := os.Getenv("MINIO_BUCKET_NAME"); bucketName != "" {
		opt.bucketName = bucketName
	}

	if accessKeyID := os.Getenv("MINIO_ACCESS_KEY_ID"); accessKeyID == "" {
		return nil, errors.New("minio access key id is not specified")
	} else {
		opt.AccessKeyID = accessKeyID
	}

	if secretAccessKey := os.Getenv("MINIO_SECRET_ACCESS_KEY"); secretAccessKey == "" {
		return nil, errors.New("minio secret access key id is not specified")
	} else {
		opt.SecretAccessKey = secretAccessKey
	}
	return &opt, nil
}

//export ReadMinioFile
func ReadMinioFile(path *C.char) (data *C.char, len C.size_t, err *C.char) {
	if opt, err := FromEnv(); err != nil {
		return nil, 0, C.CString(fmt.Errorf("bad minio env %w", err).Error())
	} else {
		minioClient, err := minio.New(opt.Endpoint, &minio.Options{
			Creds:        credentials.NewStaticV4(opt.AccessKeyID, opt.SecretAccessKey, ""),
			Secure:       opt.UseSSL,
			BucketLookup: minio.BucketLookupAuto,
		})
		if err != nil {
			return nil, 0, C.CString("create minio client error," + err.Error())
		}

		objName := C.GoString(path)
		// check object existence
		_, err = minioClient.StatObject(context.Background(), opt.bucketName, objName, minio.StatObjectOptions{})
		if err != nil {
			return nil, 0, C.CString(fmt.Errorf("error of stating object %s: %w", objName, err).Error())
		}

		fmt.Println("reading file ", C.GoString(path), opt.bucketName)
		obj, err := minioClient.GetObject(context.Background(), opt.bucketName, objName, minio.GetObjectOptions{})
		if err != nil {
			return nil, 0, C.CString(err.Error())
		} else {
			buffer := bytes.Buffer{}
			if _, err := buffer.ReadFrom(obj); err != nil {
				return nil, 0, C.CString(err.Error())
			}
			dataBytes := buffer.Bytes()
			dataLen := buffer.Len()

			p := C.malloc(C.size_t(dataLen))
			cBuf := (*[1 << 30]byte)(p)
			copy(cBuf[:], dataBytes)
			return (*C.char)(p), (C.size_t)(dataLen), C.CString("")
		}
	}
	return nil, 0, C.CString("")
}

//export ReleaseMinioFile
func ReleaseMinioFile(data *C.char) {
	defer C.free(unsafe.Pointer(data)) // remember to release memory
}

//export StopClientWrapper
func StopClientWrapper(handlesPtr uintptr) *C.char {
	return C.CString("")
}

func main() {}
