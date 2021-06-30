Import-Module "$PSScriptRoot/../siquery"

Describe 'products table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$products = $allSchemas | Where-Object { $_.BaseName -eq 'products' }
				$products | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$products = $allSchemas | Where-Object { $_.BaseName -eq 'products' }
				$schema = Get-Content $products.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$products = Get-Siq 'products' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'products' }).FullName -Raw

				$json = $products | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
