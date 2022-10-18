# Cryptography and Tokens

    "Three may keep a secret, if two of them are dead."

    – Benjamin Franklin, Poor Richard's Almanack

In this short chapter, we are going to give you a brief overview of the cryptographic services offered by the Python standard library. We are also going to touch upon JSON Web Tokens, an interesting standard for representing claims securely between two parties.

In particular, we are going to explore the following:

- Hashlib
- HMAC
- Secrets
- JSON Web Tokens with PyJWT, which seems to be the most popular Python 
library for dealing with JWTs

Let's start by taking a moment to talk about cryptography and why it is so important.

## The need for cryptography

It is estimated that, in 2021, over 4 billion people worldwide use the internet. Every year, more people are using online banking services, shopping online, or just talking to friends and family on social media. All these people expect that their money will be safe, their transactions secure, and their conversations private. 

Therefore, if you are an application developer, you have to take security very, very seriously. It doesn't matter how small or apparently insignificant your application is: security should always be a concern for you.

Security in information technology is achieved by employing several different means, but by far the most important one is cryptography. Almost everything you do with your computer or phone should include a layer where cryptography takes place. For example, cryptography is used to secure online payments, to transfer messages over a network in a way that even if someone intercepts them, they won't be able to read them, and to encrypt your files when you back them up in the cloud.

The purpose of this chapter is not to teach you all the intricacies of cryptography — there are entire books dedicated to the subject. Instead, we will show you how you can use the tools that Python offers you to create digests, tokens, and in general, to be on the safe(r) side when you need to implement something cryptography- related. As you read this chapter, it's worth bearing in mind that there is much more to cryptography than just encrypting and decrypting data; in fact, you won't find any examples of encryption or decryption in the entire chapter!

## Useful guidelines

Always remember the following rules:

- Rule number one: Do not attempt to create your own hash or encryption functions. Simply don't. Use tools and functions that are there already. It is incredibly tough to come up with a good, solid, robust algorithm to do hashing or encryption, so it's best to leave it to professional cryptographers.

- Rule number two: Follow rule number one. Those are the only two rules you need. Apart from them, it is very useful to understand cryptography, so try and learn as much as you can about this subject. There is plenty of information on the web, but for your convenience, we'll put some useful references at the end of this chapter.

Now, let's dig into the first of the standard library modules we want to show you: 
hashlib.

## Hashlib

This module provides access to a variety of cryptographic hash algorithms. These are mathematical functions that take a message of any size and produce a fixed size result, which is referred to as a hash or digest. Cryptographic hashes have many uses, from verifying data integrity to securely storing and verifying passwords.

Ideally, cryptographic hash algorithms should be:

- Deterministic: The same message should always produce the same hash.
- Irreversible: It should not be feasible to determine the original message from the hash.
- Collision resistant: It should be hard to find two different messages that produce the same hash.

These properties are crucial for the secure application of hashes. For example, it is considered imperative that passwords are only stored in hashed form. The irreversibility property ensures that even if a data breach occurs and an attacker gets hold of your password database, it would not be feasible for them to obtain the original passwords. Having the passwords stored only as hashes means that the only way to verify a user's password when they log in is to compute the hash of the password they provided and compare it against the stored hash. Of course, this will not work if the hash algorithm is not deterministic. Collision resistance is important when hashes are used for data integrity verification. If we are using a hash to check that a piece of data was not tampered with, an attacker who could find a hash collision could modify the data without changing the hash, tricking us into thinking the data was not changed.

The exact set of algorithms that are actually available through hashlib vary depending on the underlying libraries used on your platform. Some algorithms, however, are guaranteed to be present on all systems. Let's see how to find out what is available (note that your results might be different from ours):

```python
# hlib.py
>>> import hashlib
>>> hashlib.algorithms_available
{'mdc2', 'sha224', 'whirlpool', 'sha1', 'sha3_512', 'sha512_256',
 'sha256', 'md4', 'sha384', 'blake2s', 'sha3_224', 'sha3_384',
 'shake_256', 'blake2b', 'ripemd160', 'sha512', 'md5-sha1',
 'shake_128', 'sha3_256', 'sha512_224', 'md5', 'sm3'}
>>> hashlib.algorithms_guaranteed
{'blake2s', 'md5', 'sha224', 'sha3_512', 'shake_256', 'sha3_256',
 'shake_128', 'sha256', 'sha1', 'sha512', 'blake2b', 'sha3_384',
 'sha384', 'sha3_224'}
```

By opening a Python shell, we can get the set of available algorithms for our system. If our application has to talk to third-party applications, it's always best to pick an algorithm out of the guaranteed set, though, as that means every platform actually supports them. Notice that a lot of them start with sha, which stands for secure hash algorithm. 

Let's keep going in the same shell: we are going to create a hash for the byte string b'Hash me now!':
```python

>>> h = hashlib.blake2b()
>>> h.update(b'Hash me')
>>> h.update(b' now!')
>>> h.hexdigest()
'56441b566db9aafcf8cdad3a4729fa4b2bfaab0ada36155ece29f52ff70e1e9d'
'7f54cacfe44bc97c7e904cf79944357d023877929430bc58eb2dae168e73cedf'
>>> h.digest()
b'VD\x1bVm\xb9\xaa\xfc\xf8\xcd\xad:G)\xfaK+\xfa\xab\n\xda6\x15^'
b'\xce)\xf5/\xf7\x0e\x1e\x9d\x7fT\xca\xcf\xe4K\xc9|~\x90L\xf7'
b'\x99D5}\x028w\x92\x940\xbcX\xeb-\xae\x16\x8es\xce\xdf'
>>> h.block_size
128
>>> h.digest_size
64
>>> h.name
'blake2b'
```

We have used the blake2b() cryptographic function, which is quite sophisticated and was added in Python 3.6. After creating the hash object h, we update its message in two steps. Not that we need to here, but sometimes we need to hash data that is not available all at once, so it's good to know we can do it in steps.

Once we have added the entire message, we get the hexadecimal representation of the digest. This will use two characters per byte (as each character represents 4 bits, which is half a byte). We also get the byte representation of the digest, and then we inspect its details: it has a block size (the internal block size of the hash algorithm in bytes) of 128 bytes, a digest size (the size of the resulting hash in bytes) of 64 bytes, and a name. 

Let's see what we get if, instead of the blake2b() function, we use sha256():
```bash

>>> hashlib.sha256(b'Hash me now!').hexdigest()
'10d561fa94a89a25ea0c7aa47708bdb353bbb062a17820292cd905a3a60d6783'
```

The resulting hash is shorter (and therefore less secure). Notice that we can construct the hash object with the message and compute the digest in one line.

Hashing is a very interesting topic, and of course, the simple examples we've seen so far are just the start. The blake2b() function allows us a great deal of flexibility thanks to a number of parameters that can be adjusted. This means that it can be adapted for different applications or adjusted to protect against particular types of attacks. 

Here, we will just briefly discuss one of these parameters; for the full details, please refer to the official documentation at https://docs.python.org/3.7/library/hashlib.html. The person parameter is quite interesting. It is used to personalize the hash, forcing it to produce different digests for the same message. This can help to improve security when the same hash function is used for different purposes within the same application:
```python

>>> import hashlib
>>> h1 = hashlib.blake2b(b'Important data', digest_size=16,
...                      person=b'part-1')
>>> h2 = hashlib.blake2b(b'Important data', digest_size=16,
...                      person=b'part-2')
>>> h3 = hashlib.blake2b(b'Important data', digest_size=16)
>>> h1.hexdigest()
'c06b9af95d5aa6307e7e3fd025a15646'
>>> h2.hexdigest()
'9cb03be8f3114d0f06bddaedce2079c4'
>>> h3.hexdigest()
'7d35308ca3b042b5184728d2b1283d0d'
```

Here we've also used the digest_size parameter to get hashes that are only 16 bytes long.

General-purpose hash functions, like blake2b() or sha256(), are not suitable for securely storing passwords. General-purpose hash functions are quite fast to compute on modern computers, which makes it feasible for an attacker to reverse the hash by brute force (trying millions of possibilities per second until they find a match). Key derivation algorithms like pbkdf2_hmac() are designed to be slow enough to make such brute-force attacks infeasible. The pbkdf2_hmac() key derivation algorithm achieves this by using many repeated applications of a general- purpose hash function (the number of iterations can be specified as a parameter). As computers get more and more powerful, it is important to increase the number of iterations we do over time, otherwise the likelihood of a successful brute-force attack on our data increases as time passes. Good password hash functions should also use salt. Salt is a random piece of data used to initialize the hash function; this randomizes the output of the algorithm and protects against attacks where hashes are compared to tables of known hashes. The pbkdf2_hmac() function supports salting via a required salt parameter.

Here's how you can use pbkdf2_hmac() to hash a password:

```python
>>> import os
>>> dk = hashlib.pbkdf2_hmac('sha256', b'Password123',
...   salt=os.urandom(16), iterations=100000
... )
>>> dk.hex()
'f8715c37906df067466ce84973e6e52a955be025a59c9100d9183c4cbec27a9e'
```

Notice that we have used os.urandom() to provide a 16-byte random salt, as recommended by the documentation.

We encourage you to explore and experiment with this module, as sooner or later you will have to use it. Now, let's move on to the hmac module.

## HMAC

This module implements the HMAC algorithm, as described by RFC 2104 (https:// tools.ietf.org/html/rfc2104.html). HMAC (which stands for hash-based message authentication code or keyed-hash message authentication code, depending on who you ask) is a widely used mechanism for authenticating messages and verifying that they have not been tampered with. 

The algorithm combines a message with a secret key and generates a hash of the combination. This hash is referred to as a message authentication code (MAC) or signature. The signature is stored or transmitted along with the message. At a later time, you can verify that the message has not been tampered with by re-computing the signature using the same secret key and comparing it to the previously computed signature. The secret key must be carefully protected, otherwise an attacker with access to the key would be able to modify the message and replace the signature, thereby defeating the authentication mechanism.

Let's see a small example of how to compute a message authentication code:

# hmc.py
import hmac
import hashlib

def calc_digest(key, message):
    key = bytes(key, 'utf-8')
    message = bytes(message, 'utf-8')
    dig = hmac.new(key, message, hashlib.sha256)
    return dig.hexdigest()

mac = calc_digest('secret-key', 'Important Message')

The hmac.new() function takes a secret key, a message, and the hash algorithm to use and returns an hmac object, which has a similar interface to the hash objects from hashlib. The key must be a bytes or bytearray object and the message can be any bytes-like object. Therefore, we convert our key and the message into bytes before creating an hmac instance (dig), which we use to get a hexadecimal representation of the hash.

We'll see a bit more of how HMAC signatures can be used later in this chapter, when we talk about JWTs. Before that, however, we'll take a quick look at the secrets module.

## Secrets

This small module was added in Python 3.6 and deals with three things: random numbers, tokens, and digest comparison. It uses the most secure random number generators provided by the underlying operating system to generate tokens and random numbers suitable for use in cryptographic applications. Let's have a quick look at what it provides.

### Random numbers

We can use three functions in order to deal with random numbers:

```python
# secrs/secr_rand.py
import secrets
print(secrets.choice('Choose one of these words'.split()))
print(secrets.randbelow(10 ** 6))
print(secrets.randbits(32))
```

The first one, choice(), picks an element at random from a non-empty sequence. The second, randbelow(), generates a random integer between 0 and the argument you call it with, and the third, randbits(), generates an integer with the given number of random bits in it. Running that code produces the following output (which will of course be different every time it is run):

```bash
$ python secr_rand.py
one
504156
3172492450
```

You should use these functions instead of those from the random module whenever you need randomness in the context of cryptography, as these are specially designed for this task. Let's see what the module gives us for tokens.

## Token generation

Again, we have three functions for generating tokens, each in a different format. Let's see the example:

```python
# secrs/secr_rand.py
print(secrets.token_bytes(16))
print(secrets.token_hex(32))
print(secrets.token_urlsafe(32))
```

The token_bytes() function simply returns a random byte string containing the specified number of bytes (16, in this example). The other two do the same, but token_hex() returns a token in hexadecimal format, and token_urlsafe() returns a token that only contains characters suitable for being included in a URL. Let's see the output (which is a continuation from the previous run):
```python

b'\xda\x863\xeb\xbb|\x8fk\x9b\xbd\x14Q\xd4\x8d\x15}'
9f90fd042229570bf633e91e92505523811b45e1c3a72074e19bbeb2e5111bf7
bl4qz_Av7QNvPEqZtKsLuTOUsNLFmXW3O03pn50leiY

```

Let's see how we can use these tools to write our own random password generator:
```python

# secrs/secr_gen.py
import secrets
from string import digits, ascii_letters

def generate_pwd(length=8):
    chars = digits + ascii_letters
    return ''.join(secrets.choice(chars) for c in range(length))

def generate_secure_pwd(length=16, upper=3, digits=3):
    if length < upper + digits + 1:
        raise ValueError('Nice try!')
    while True:
        pwd = generate_pwd(length)
        if (any(c.islower() for c in pwd)
            and sum(c.isupper() for c in pwd) >= upper
            and sum(c.isdigit() for c in pwd) >= digits):
            return pwd

print(generate_secure_pwd())
print(generate_secure_pwd(length=3, upper=1, digits=1))
```

Our generate_pwd() function simply generates a random string of a given length by joining together length characters picked at random from a string that contains all the letters of the alphabet (lowercase and uppercase), and the 10 decimal digits.

Then, we define another function, generate_secure_pwd(), that simply keeps calling generate_pwd() until the random string we get matches some simple requirements. The password must be length characters long, have at least one lowercase character, upper uppercase characters, and digits digits.

If the total number of uppercase characters, lowercase characters, and digits specified by the parameters is greater than the length of the password we are generating, we can never satisfy the conditions. So, in order to avoid getting stuck in an infinite loop, we have put a check clause in the first line of the body, and we raise a ValueError if the requirements cannot be satisfied.

The body of the while loop is straightforward: first we generate the random password, and then we verify the conditions by using any() and sum(). The any function returns True if any of the items in the iterable it's called with evaluate to True. The use of sum() is actually slightly trickier here, in that it exploits polymorphism. As you may recall from Chapter 2, Built-In Data Types, the bool type is a subclass of int, therefore when summing on an iterable of True and False values, they will automatically be interpreted as integers (with the values 1 and 0) by the sum() function. This is an example of polymorphism, which we briefly discussed in Chapter 6, OOP, Decorators, and Iterators.

Running the example produces the following result:
```bash

$ python secr_gen.py
nsL5voJnCi7Ote3F
J5e

```

That second password is probably not very secure...

One common use of random tokens is in password reset URLs for websites. Let's see an example of how we can generate such a URL:
```python

# secrs/secr_reset.py
import secrets

def get_reset_pwd_url(token_length=16):
    token = secrets.token_urlsafe(token_length)
    return f'https://example.com/reset-pwd/{token}'

print(get_reset_pwd_url())

```

This function is so easy we will only show you the output:
```bash

$ python secr_reset.py
https://example.com/reset-pwd/dfVPEPl_pCkQ8YNV4er-UQ

```

## Digest comparison

This is probably quite surprising, but the secrets module also provides a compare_ digest(a, b) function, which is the equivalent of comparing two digests by simply doing a == b. So, why would we need that function? It's because it has been designed to prevent timing attacks. These kinds of attacks can infer information about where the two digests start being different, according to the time it takes for the comparison to fail. So, compare_digest() prevents this attack by removing the correlation between time and failures. We think this is a brilliant example of how sophisticated attacking methods can be. If you raised your eyebrows in astonishment, maybe now it's clearer why we said never to implement cryptography functions by yourself.

This brings us to the end of our tour of the cryptographic services in the Python standard library. Now, let's move on to a different type of token: JWTs.

## JSON Web Tokens

A JSON Web Token, or JWT, is a JSON-based open standard for creating tokens that assert some number of claims. JWTs are frequently used as authentication tokens. In this context, the claims are typically statements about the identity and permissions of an authenticated user. The tokens are cryptographically signed, which makes it possible to verify that the content of the token has not been modified since it was issued. You can learn all about this technology on the website (https://jwt.io/).

This type of token is comprised of three sections, separated by a dot, in the format A.B.C. B is the payload, which is where we put the claims. C is the signature, which is used to verify the validity of the token, and A is a header, which identifies the token as a JWT, and indicates the algorithm used to compute the signature. A, B, and C are all encoded with a URL-safe Base64 encoding (which we'll refer to as Base64URL). The Base64URL encoding makes it possible to use JWTs as part of URLs (typically as query parameters); however, JWTs do also appear in many other places, including HTTP headers.

Base64 is a very popular binary-to-text encoding scheme that represents binary data in an ASCII string format by translating it into a radix-64 representation. The radix-64 representation uses the letters A-Z, a-z, and the digits 0-9, plus the two symbols + and / for a total of 64 symbols. Base64 is used, for example, to encode images attached in an email. It happens seamlessly, so the vast majority of people are completely oblivious to this fact. Base64URL is a variant of Base64 encoding where the + and / characters (which have specific meanings in the context of a URL) are replaced with - and _. The = character (which is used for padding in Base64) also has a special meaning within URLs and is omitted in Base64URL.

The way this type of token works is slightly different to what we have seen so far in this chapter. In fact, the information that the token carries is always visible. You just need to decode A and B from Base64URL to get the algorithm and the payload. The security lies in part C, which is an HMAC signature of the header and payload. If you try to modify either the A or B part by editing the header or the payload, encoding it back to Base64URL, and replacing it in the token, the signature won't match, and therefore the token will be invalid. This means that we can build a payload with claims such as logged in as admin, or something along those lines, and as long as the token is valid, we know we can trust that that user is actually logged in as an admin.

When dealing with JWTs, you want to make sure you have researched how to handle them safely. Things like not accepting unsigned tokens, or restricting the list of algorithms you use to encode and decode, as well as other security measures, are very important and you should take the time to investigate and learn them.

For this part of the code, you will have to have the PyJWT and cryptography Python packages installed. As always, you will find them in the requirements of the source code for this chapter.

Let's start with a simple example:

```python
# jwt/tok.py
import jwt

data = {'payload': 'data', 'id': 123456789}

token = jwt.encode(data, 'secret-key')
algs = ['HS256', 'HS512']
data_out = jwt.decode(token, 'secret-key', algorithms=algs)
print(token)
print(data_out)
```

We define the data payload, which contains an ID and some payload data. We create a token using the jwt.encode() function, which takes the payload and a secret key. The secret key is used to generate the HMAC signature of the token header and payload. Next, we decode the token again, specifying the signature algorithms that we are willing to accept. The default algorithm used to calculate the token is HS256; in this example, we accept either HS256 or HS512 when decoding (if the token had been generated using a different algorithm, it would be rejected with an exception). Let's see the output:
```python

$ python tok.py
b'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.
eyJwYXlsb2FkIjoiZGF0YSIsImlkIjoxMjM0NTY3ODl9.WFRY-uoACMoNYX97PXXjEfXFQO
1rCyFCyiwxzOVMn40'
{'payload': 'data', 'id': 123456789}

```

As you can see, the token is a binary string of Base64URL-encoded pieces of data. We called jwt.decode(), providing the correct secret key. If we had supplied the wrong key, we would have gotten an error, since the signature can only be verified with the same secret that was used to generate it.

JWTs are often used to transmit information between two parties. For example, authentication protocols that allow websites to rely on third-party identity providers to authenticate users often use JWTs. In such cases, the secret key used to sign tokens needs to be shared between the two parties. Therefore, it is often referred to as a shared secret.

Care must be taken to protect the shared secret, since anyone with access to it can generate valid tokens.

Sometimes, you might want to be able to inspect the content of the token without verifying the signature first. You can do so by simply calling decode() this way:
```python

# jwt/tok.py
jwt.decode(token, options={'verify_signature': False})

```

This is useful, for example, when values in the token payload are needed to recover the secret key, but that technique is quite advanced so we won't be spending time on it in this context. Instead, let's see how we can specify a different algorithm for computing the signature:
```python

# jwt/tok.py
token512 = jwt.encode(data, 'secret-key', algorithm='HS512')
data_out = jwt.decode(token512, 'secret-key', algorithms=['HS512'])
print(data_out)

```

Here we have used the HS512 algorithm to generate the token and on decoding specified that we would only accept tokens generated using the HS512 algorithm. The output is our original payload dictionary.

Now, while you are free to put whatever you want in the token payload, there are some claims that have been standardized, and they enable you to have a great deal of control over the token.

## Registered claims

The JWT standard defines the following official registered claims:

-  iss: The issuer of the token
-  sub: The subject information about the party this token is carrying information about
-  aud: The audience for the token
-  exp: The expiration time, after which the token is considered to be invalid
-  nbf: The not before (time), or the time before which the token is not considered to be valid yet
-  iat: The time at which the token was issued
-  jti: The token ID

Claims that are not defined in the standard can be categorized as public or private:

•  Public: Claims that are publicly allocated for a particular purpose. Public 

claim names can be reserved by registering them with the IANA JSON Web 
Token Claims Registry. Alternatively, the claims should be named in a way 
that ensures that they do not clash with any other public or official claim 
names (one way of achieving this could be to prepend a registered domain 
name to the claim name).

•  Private: Any other claims that do not fall under the above categories are 

referred to as private claims. The meaning of such claims is typically defined 
within the context of a particular application and they are meaningless 
outside that context. To avoid ambiguity and confusion, care must be 
taken to avoid name clashes.

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.

d
e

t
i

m
L

i

 
,

g
n
h
s

i

i
l

b
u
P

 
t
k
c
a
P

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

Romano, Fabrizio, and Heinrich Kruger. Learn Python Programming : An in-Depth Introduction to the Fundamentals of Python, 3rd Edition, Packt Publishing,
         Limited, 2021. ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6797218.
Created from th-deggendorf on 2022-03-05 19:21:52.

[ 307 ]

Chapter 9To learn all about claims, please refer to the official website. Now, let's see a couple 
of code examples involving a subset of these claims.

Time-related claims
Let's see how we might use the claims related to time:

# jwt/claims_time.py
from datetime import datetime, timedelta, timezone
from time import sleep, time
import jwt

iat = datetime.now(tz=timezone.utc)
nfb = iat + timedelta(seconds=1)
exp = iat + timedelta(seconds=3)
data = {'payload': 'data', 'nbf': nfb, 'exp': exp, 'iat': iat}

def decode(token, secret):
    print(time())
    try:
        print(jwt.decode(token, secret, algorithms=['HS256']))
    except (
        jwt.ImmatureSignatureError, jwt.ExpiredSignatureError
    ) as err:
        print(err)
        print(type(err))

secret = 'secret-key'
token = jwt.encode(data, secret)

decode(token, secret)
sleep(2)
decode(token, secret)
sleep(2)
decode(token, secret)

In this example, we set the issued at (iat) claim to the current UTC time (UTC 
stands for Coordinated Universal Time). We then set the not before (nbf) and expire 
time (exp) at 1 and 3 seconds from now, respectively. We define a decode() helper 
function that reacts to a token not being valid yet, or being expired, by trapping the 
appropriate exceptions, and then we call it three times, interspersed by two calls 
to sleep(). 

[ 308 ]

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.

d
e

t
i

m
L

i

 
,

g
n
h
s

i

i
l

b
u
P

 
t
k
c
a
P

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

Romano, Fabrizio, and Heinrich Kruger. Learn Python Programming : An in-Depth Introduction to the Fundamentals of Python, 3rd Edition, Packt Publishing,
         Limited, 2021. ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6797218.
Created from th-deggendorf on 2022-03-05 19:21:52.

Cryptography and TokensThis way, we will try to decode the token before it is valid, then when it is valid, 
and finally after it has expired. This function also prints a useful timestamp before 
attempting to decode the token. Let's see how it goes (blank lines have been added 
for readability):

$ python jwt/claims_time.py
1631043839.6459477
The token is not yet valid (nbf)
<class 'jwt.exceptions.ImmatureSignatureError'>

1631043841.6480813
{'payload': 'data', 'nbf': 1631043840, 'exp': 1631043842, 'iat':
1631043839}

1631043843.6498601
Signature has expired
<class 'jwt.exceptions.ExpiredSignatureError'>

As you can see, it all executed as expected. We get descriptive messages from the 
exceptions and get the original payload back when the token is actually valid.

Authentication-related claims
Let's see another quick example involving the issuer (iss) and audience (aud) claims. 
The code is conceptually very similar to the previous example, and we're going to 
exercise it in the same way:

# jwt/claims_auth.py
import jwt

data = {'payload': 'data', 'iss': 'hein', 'aud': 'learn-python'}

secret = 'secret-key'
token = jwt.encode(data, secret)

def decode(token, secret, issuer=None, audience=None):
    try:
        print(jwt.decode(token, secret, issuer=issuer,
                         audience=audience, algorithms=["HS256"]))
    except (
        jwt.InvalidIssuerError, jwt.InvalidAudienceError
    ) as err:
        print(err)

[ 309 ]

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.

d
e

t
i

m
L

i

 
,

g
n
h
s

i

i
l

b
u
P

 
t
k
c
a
P

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

Romano, Fabrizio, and Heinrich Kruger. Learn Python Programming : An in-Depth Introduction to the Fundamentals of Python, 3rd Edition, Packt Publishing,
         Limited, 2021. ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6797218.
Created from th-deggendorf on 2022-03-05 19:21:52.

Chapter 9        print(type(err))

decode(token, secret)

# not providing the issuer won't break
decode(token, secret, audience='learn-python')

# not providing the audience will break
decode(token, secret, issuer='hein')

# both will break
decode(token, secret, issuer='wrong', audience='learn-python')
decode(token, secret, issuer='hein', audience='wrong')

decode(token, secret, issuer='hein', audience='learn-python')

As you can see, this time, we have specified issuer and audience. It turns out that 
if we don't provide the issuer when decoding the token, it won't cause the decoding 
to break. However, providing the wrong issuer will actually break decoding. On the 
other hand, both failing to provide the audience, or providing the wrong audience, 
will break decoding.

As in the previous example, we have written a custom decode() function that reacts 
to the appropriate exceptions. See if you can follow along with the calls and the 
relative output that follows (we'll help with some blank lines):

$ python jwt/claims_time.py
Invalid audience
<class 'jwt.exceptions.InvalidAudienceError'>

{'payload': 'data', 'iss': 'hein', 'aud': 'learn-python'}

Invalid audience
<class 'jwt.exceptions.InvalidAudienceError'>

Invalid issuer
<class 'jwt.exceptions.InvalidIssuerError'>

Invalid audience
<class 'jwt.exceptions.InvalidAudienceError'>

{'payload': 'data', 'iss': 'hein', 'aud': 'learn-python'}

[ 310 ]

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.

d
e

t
i

m
L

i

 
,

g
n
h
s

i

i
l

b
u
P

 
t
k
c
a
P

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

Romano, Fabrizio, and Heinrich Kruger. Learn Python Programming : An in-Depth Introduction to the Fundamentals of Python, 3rd Edition, Packt Publishing,
         Limited, 2021. ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6797218.
Created from th-deggendorf on 2022-03-05 19:21:52.

Cryptography and TokensNow, let's see one final example for a more complex use case.

Using asymmetric (public key) algorithms
Sometimes, using a shared secret is not the best option. In such cases, it is possible to 
use an asymmetric key pair instead of HMAC to generate the JWT signature. In this 
example, we are going to create a token (and decode it) using an RSA key pair.
Public key cryptography, or asymmetrical cryptography, is any cryptographic 
system that uses pairs of keys: public keys which may be disseminated widely, and 
private keys which are known only to the owner. If you are interested in learning 
more about this topic, please see the end of this chapter for recommendations. A 
signature can be generated using the private key, and the public key can be used to 
verify the signature. Thus, two parties can exchange JWTs and the signatures can be 
verified without any need for a shared secret.

Now, let's create an RSA key pair. We're going to use the ssh-keygen utility from 
OpenSSH (https://www.ssh.com/ssh/keygen/) to do this. In the folder where 
our scripts for this chapter are, we created a jwt/rsa subfolder. Within it, run the 
following:

$ ssh-keygen -t rsa –m PEM

Give the name key when asked for a filename (it will be saved in the current folder), 
and simply hit the Enter key when asked for a passphrase.
Having generated our keys, we can now change back to the ch09 folder, and run this 
code:

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.

d
e

t
i

m
L

i

 
,

g
n
h
s

i

i
l

b
u
P

 
t
k
c
a
P

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

# jwt/token_rsa.py
import jwt

data = {'payload': 'data'}

def encode(data, priv_filename, algorithm='RS256'):
    with open(priv_filename, 'rb') as key:
        private_key = key.read()
    return jwt.encode(data, private_key, algorithm=algorithm)

def decode(data, pub_filename, algorithm='RS256'):
    with open(pub_filename, 'rb') as key:
        public_key = key.read()
    return jwt.decode(data, public_key, algorithms=[algorithm])

[ 311 ]

Romano, Fabrizio, and Heinrich Kruger. Learn Python Programming : An in-Depth Introduction to the Fundamentals of Python, 3rd Edition, Packt Publishing,
         Limited, 2021. ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6797218.
Created from th-deggendorf on 2022-03-05 19:21:52.

Chapter 9token = encode(data, 'jwt/rsa/key')
data_out = decode(token, 'jwt/rsa/key.pub')
print(data_out)

In this example, we defined a couple of custom functions to encode and decode 
tokens using private/public keys. As you can see in the encode() function, we are 
using the RS256 algorithm this time. Notice that when we encode, we provide the 
private key, which is used to generate the JWT signature. When we decode the JWT, 
we instead supply the public key, which is used to verify the signature.

The logic is pretty straightforward, and we would encourage you to think about 
at least one use case where this technique might be more suitable than using 
a shared key.

Useful references

Here, you can find a list of useful references if you want to dig deeper into the 
fascinating world of cryptography:

•  Cryptography: https://en.wikipedia.org/wiki/Cryptography
• 
•  RFC standard for JSON Web Tokens: https://datatracker.ietf.org/doc/

JSON Web Tokens: https://jwt.io

html/rfc7519

•  Hash functions: https://en.wikipedia.org/wiki/Cryptographic_hash_

function

•  HMAC: https://en.wikipedia.org/wiki/HMAC
•  Cryptography services (Python STD library): https://docs.python.org/3.7/

• 

library/crypto.html
IANA JSON Web Token Claims Registry: https://www.iana.org/
assignments/jwt/jwt.xhtml

•  PyJWT library: https://pyjwt.readthedocs.io/
•  Cryptography library: https://cryptography.io/

There is way more information on the web, and plenty of books you can also study, 
but we'd recommend that you start with the main concepts and then gradually dive 
into the specifics you want to understand more thoroughly.

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.

d
e

t
i

m
L

i

 
,

g
n
h
s

i

i
l

b
u
P

 
t
k
c
a
P

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

Romano, Fabrizio, and Heinrich Kruger. Learn Python Programming : An in-Depth Introduction to the Fundamentals of Python, 3rd Edition, Packt Publishing,
         Limited, 2021. ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6797218.
Created from th-deggendorf on 2022-03-05 19:21:52.

[ 312 ]

Cryptography and TokensSummary

In this short chapter, we explored the world of cryptography in the Python standard 
library. We learned how to create a hash (or digest) for a message using different 
cryptographic functions. We also learned how to create tokens and deal with random 
data when it comes to the cryptography context.

We then took a small tour outside the standard library to learn about JSON 
Web Tokens, which are commonly used in authentication and claims-related 
functionalities by modern systems and applications.

The most important thing is to understand that doing things manually can be 
very risky when it comes to cryptography, so it's always best to leave it to the 
professionals and simply use the tools we have available.

The next chapter will be about testing our code so that we can be confident that 
it works the way it is supposed to.

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.

d
e

t
i

m
L

i

 
,

g
n
h
s

i

i
l

b
u
P

 
t
k
c
a
P

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

Romano, Fabrizio, and Heinrich Kruger. Learn Python Programming : An in-Depth Introduction to the Fundamentals of Python, 3rd Edition, Packt Publishing,
         Limited, 2021. ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6797218.
Created from th-deggendorf on 2022-03-05 19:21:52.

[ 313 ]

Chapter 9.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.

d
e

t
i

m
L

i

 
,

g
n
h
s

i

i
l

b
u
P

 
t
k
c
a
P

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

Romano, Fabrizio, and Heinrich Kruger. Learn Python Programming : An in-Depth Introduction to the Fundamentals of Python, 3rd Edition, Packt Publishing,
         Limited, 2021. ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6797218.
Created from th-deggendorf on 2022-03-05 19:21:52.


